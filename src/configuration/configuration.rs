use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub input: ConfigurationInput,
    pub outputs: Vec<ConfigurationOutput>
}

#[derive(Deserialize, Debug)]
pub struct ConfigurationInput {
    pub path: String
}

#[derive(Deserialize, Debug)]
pub struct ConfigurationOutput {
    pub path: String
}
