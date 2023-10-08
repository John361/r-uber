use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum UberOutput {
    Local {
        path: String
    },
    Sftp {
        login: String,
        #[serde(rename = "authenticationMethod")] authentication_method: UberOutputSftpAuthenticationMethod,
        #[serde(rename = "remotePath")] remote_path: String
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum UberOutputSftpAuthenticationMethod {
    Password(String),
    Key {
        #[serde(rename = "publicKeyPath")] public_key_path: String,
        #[serde(rename = "privateKeyPath")] private_key_path: String
    }
}
