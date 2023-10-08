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

    pub fn from_settings(file_path: &str) -> Self {
        Config::builder()
            .add_source(config::File::with_name(file_path))
            .build()
            .expect("Cannot deserialize configuration")
            .try_deserialize()
            .expect("Cannot deserialize configuration")
    }
}
