use serde::{Serialize, Deserialize};
use serde_json::{from_str as serde_json_from_str, to_string as serde_json_to_string};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub input: ConfigurationInput,
    pub outputs: Vec<ConfigurationOutput>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationInput {
    pub path: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurationOutput {
    pub path: String
}

impl Configuration {

    pub fn to_string(&self) -> String {
        serde_json_to_string(self)
            .expect("Cannot serialize configuration")
    }

    pub fn from_string(content: &str) -> Self {
        serde_json_from_str(content)
            .expect("Cannot deserialize configuration")
    }
}
