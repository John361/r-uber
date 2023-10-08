use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UberOutput {
    Local {
        path: String,
    },
    Sftp {
        login: String,
        #[serde(rename = "authenticationMethod")]
        authentication_method: UberOutputSftpAuthenticationMethod,
        #[serde(rename = "remotePath")]
        remote_path: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub fn take_passenger_and_drive_to(&self, input_file_path: &str) {
        match &self {
            UberOutput::Local { path } => {
                println!(
                    "Should copy file locally from {} to {}",
                    input_file_path, path
                );
            }

            UberOutput::Sftp {
                login,
                authentication_method,
                remote_path,
            } => {
                println!(
                    "Should copy file remotely from {} to {} using login {} with auth method {:?}",
                    input_file_path, remote_path, login, authentication_method
                );
            }
        }
    }
}
