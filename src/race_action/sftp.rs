use std::net::TcpStream;
use std::path::Path;

use ssh;
use ssh::SessionConnector;

use crate::logger;
use crate::race::uber_output::{UberOutput, UberOutputSftpAuthenticationMethod};
use crate::race_action::RaceActionError;

pub fn copy(input: &Path, output: &UberOutput) -> Result<bool, RaceActionError> {
    let session = create_ssh_session(output).map_err(|error| {
        logger::warn("sftp", "copy", &error.to_string());
        error
    })?;

    let scp = session.run_local().open_scp().map_err(|error| {
        logger::warn("sftp", "copy", &error.to_string());
        RaceActionError::SessionUsage(error.to_string())
    })?;

    match output {
        UberOutput::Sftp { remote_path, .. } => {
            scp.upload(input, remote_path.as_ref()).map_err(|error| {
                logger::warn("sftp", "copy", &error.to_string());
                RaceActionError::Copy {
                    source_path: input.to_string_lossy().to_string(),
                    destination: remote_path.to_string(),
                }
            })?;

            let success_message: String = format!(
                "Successfully remotely copy file from {:?} to {}",
                input, &remote_path
            );
            logger::info("local", "copy", &success_message);
            Ok(true)
        }

        _ => {
            logger::warn("sftp", "copy", "Wrong output used with method");
            Err(RaceActionError::WrongOutput)
        }
    }
}

fn create_ssh_session(output: &UberOutput) -> Result<SessionConnector<TcpStream>, RaceActionError> {
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

            let session =
                session.map_err(|error| RaceActionError::SessionUsage(error.to_string()))?;

            Ok(session)
        }

        _ => Err(RaceActionError::WrongOutput),
    }
}
