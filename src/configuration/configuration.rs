use serde::{Serialize, Deserialize};

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
