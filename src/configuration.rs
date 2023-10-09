use config::Config;
use serde::Deserialize;

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

impl Configuration {
    pub fn from_file(file_path: &str) -> Result<Self, String> {
        let config = Config::builder()
            .add_source(config::File::with_name(file_path))
            .build();

        match config {
            Ok(config) => {
                match config.try_deserialize::<Configuration>() {
                    Ok(configuration) => {
                        logger::info("configuration", "from_file", "Successfully parsed configuration file");
                        Ok(configuration)
                    }
                    Err(_) => {
                        logger::error("configuration", "from_file", "Cannot deserialize configuration file");
                        Err("Cannot deserialize configuration file".to_string())
                    }
                }
            }

            Err(error) => {
                let error_message: String = format!("Cannot build configuration file: {}", error);
                logger::error("configuration", "from_file", &error_message);
                Err(error_message)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_loaded_and_parsed() {
        let file_path: &str = "./tests/config.json";
        let _result: Configuration = Configuration::from_file(file_path).expect("");
    }
}
