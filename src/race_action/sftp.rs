use std::net::TcpStream;
use std::path::Path;

use ssh;
use ssh::{SessionConnector, SshResult};

use crate::race::uber_output::{UberOutput, UberOutputSftpAuthenticationMethod};

pub fn copy(input: &Path, output: &UberOutput) -> SshResult<()> {
    let session = create_ssh_session(output)
        .unwrap()
        .run_local()
        .open_scp()
        .unwrap();

    session.upload(input, "./".as_ref())
}

fn create_ssh_session(output: &UberOutput) -> Option<SessionConnector<TcpStream>> {
    match output {
        UberOutput::Sftp { host, port, login, authentication_method, .. } => {
            match authentication_method {
                UberOutputSftpAuthenticationMethod::Password(password) => {
                    Some(ssh::create_session()
                        .username(login)
                        .password(password)
                        .connect(format!("{}:{}", host, port))
                        .expect("Cannot connect to remote host"))
                }

                UberOutputSftpAuthenticationMethod::Key { private_key_path, .. } => {
                    Some(ssh::create_session()
                        .username(login)
                        .private_key_path(private_key_path)
                        .connect(format!("{}:{}", host, port))
                        .expect("Cannot connect to remote host"))
                }
            }
        }

        _ => { None }
    }
}
