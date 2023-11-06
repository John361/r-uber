use config::Config;
use serde::Deserialize;
use thiserror::Error;

use crate::logger;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub app_name: String,
    pub races_path: String,
    pub kafka: ConfigKafka,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ConfigKafka {
    pub hosts: Vec<String>,
    pub topic: String,
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Cannot parse configuration: {0}")]
    CannotParse(String),
}

impl Configuration {
    pub fn from_file(file_path: &str) -> Result<Self, ConfigurationError> {
        let config = Config::builder()
            .add_source(config::File::with_name(file_path))
            .build()
            .map_err(|error| {
                logger::error("configuration", "from_file", &error.to_string());
                ConfigurationError::CannotParse(error.to_string())
            })?;

        let result = config.try_deserialize::<Configuration>().map_err(|error| {
            logger::error("configuration", "from_file", &error.to_string());
            ConfigurationError::CannotParse(error.to_string())
        })?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_loaded_and_parsed() {
        let file_path: &str = "./tests/config.json";
        let _result: Configuration = Configuration::from_file(file_path).unwrap();
    }
}
