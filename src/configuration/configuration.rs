use config::Config;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub races_path: String,
    pub kafka: ConfigKafka
}

#[derive(Deserialize, Debug)]
pub struct ConfigKafka {
    pub hosts: Vec<String>,
    pub topic: String
}

impl Configuration {

    pub fn from_file(config_path: &str) -> Self {
        Config::builder()
            .add_source(config::File::with_name(config_path))
            .build()
            .expect("Cannot deserialize configuration")
            .try_deserialize()
            .expect("Cannot deserialize configuration")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn config_loaded_and_parsed() {
        let config_path: &str = "./tests/config.json";
        let _config: Configuration = Configuration::from_file(config_path);
    }
}
