use serde::{Deserialize, Serialize};

use crate::race::uber::Uber;
use crate::race_action::local;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum UberOutput {
    Local {
        path: String,
    },
    Sftp {
        host: String,
        port: String,
        login: String,
        #[serde(rename = "authenticationMethod")]
        authentication_method: UberOutputSftpAuthenticationMethod,
        #[serde(rename = "remotePath")]
        remote_path: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum UberOutputSftpAuthenticationMethod {
    Password(String),
    Key {
        #[serde(rename = "publicKeyPath")]
        public_key_path: String,
        #[serde(rename = "privateKeyPath")]
        private_key_path: String,
    },
}

impl UberOutput {
    pub fn take_passenger_and_drive_to(&self, uber: &Uber, passenger: &str) {
        match &self {
            UberOutput::Local { path } => {
                if let Err(error) = local::copy(passenger.as_ref(), self) {
                    println!(
                        "Error occured when copy from {} to {}: {:?}",
                        passenger, path, error
                    );
                } else {
                    println!("Successfully copy from {} to {}", passenger, path);
                }
            }

            UberOutput::Sftp {
                host,
                port,
                login,
                authentication_method,
                remote_path,
            } => {
                println!(
                    "Should copy file remotely from {} to {} using login {} with auth method {:?}",
                    uber.input.path, remote_path, login, authentication_method
                );
            }
        }
    }
}
