use std::net::TcpStream;
use std::path::Path;

use ssh;
use ssh::SessionConnector;

use crate::logger;
use crate::race::uber_output::{UberOutput, UberOutputSftpAuthenticationMethod};

pub fn copy(input: &Path, output: &UberOutput) -> Result<bool, String> {
    match create_ssh_session(output) {
        Ok(session) => {
            match session.run_local().open_scp() {
                Ok(scp) => {
                    if let Err(error) = scp.upload(input, "./".as_ref()) {
                        // TODO: fix path
                        let error_message = format!("Cannot open scp: {}", error);
                        logger::warn("sftp", "copy", &error_message);
                        Err(error_message)
                    } else {
                        let success_message: String = format!(
                            "Successfully remotely copy file from {:?} to {}",
                            input, "./"
                        ); // TODO: fix path
                        logger::info("local", "copy", &success_message);
                        Ok(true)
                    }
                }

                Err(error) => {
                    let error_message = format!("Cannot open scp: {}", error);
                    logger::warn("sftp", "copy", &error_message);
                    Err(error_message)
                }
            }
        }

        Err(error) => {
            logger::warn("sftp", "copy", &error);
            Err(error)
        }
    }
}

fn create_ssh_session(output: &UberOutput) -> Result<SessionConnector<TcpStream>, String> {
    match output {
        UberOutput::Sftp {
            host,
            port,
            login,
            authentication_method,
            ..
        } => {
            let session = match authentication_method {
                UberOutputSftpAuthenticationMethod::Password(password) => ssh::create_session()
                    .username(login)
                    .password(password)
                    .connect(format!("{}:{}", host, port)),

                UberOutputSftpAuthenticationMethod::Key {
                    private_key_path, ..
                } => ssh::create_session()
                    .username(login)
                    .private_key_path(private_key_path)
                    .connect(format!("{}:{}", host, port)),
            };

            match session {
                Ok(session) => Ok(session),

                Err(_) => {
                    let error_message = format!(
                        "Cannot open remote session for {} on {}:{}",
                        login, host, port
                    );
                    logger::warn("sftp", "create_ssh_session", &error_message);
                    Err(error_message)
                }
            }
        }

        _ => {
            let error_message: String = format!("Wrong output used with the method: {:?}", output);
            logger::warn("sftp", "create_ssh_session", &error_message);
            Err(error_message)
        }
    }
}
