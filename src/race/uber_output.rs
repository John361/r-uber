use crate::logger;
use serde::{Deserialize, Serialize};

use crate::race::uber::Uber;
use crate::race_action::{local, sftp};

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
    pub fn take_passenger_and_drive_to(&self, _: &Uber, passenger: &str) {
        match &self {
            UberOutput::Local { .. } => {
                if let Err(error) = local::copy(passenger.as_ref(), self) {
                    let error_message = format!("Error occurred with passenger: {}", error);
                    logger::warn("uber_output", "take_passenger_and_drive_to", &error_message);
                } else {
                    logger::info(
                        "uber_output",
                        "take_passenger_and_drive_to",
                        "Successfully drive passenger to destination",
                    );
                }
            }

            UberOutput::Sftp { .. } => {
                if let Err(error) = sftp::copy(passenger.as_ref(), self) {
                    let error_message = format!("Error occurred with passenger: {}", error);
                    logger::warn("uber_output", "take_passenger_and_drive_to", &error_message);
                } else {
                    logger::info(
                        "uber_output",
                        "take_passenger_and_drive_to",
                        "Successfully drive passenger to destination",
                    );
                }
            }
        }
    }
}
