use std::fs;

use serde_json::{from_str as serde_json_from_str};
use crate::configuration::configuration::Configuration;

pub fn read(configuration_file_path: &str) -> Configuration {
    let error_message: String = format!("Cannot read configurations file from path {0}",
                                        configuration_file_path);

    let content: String = fs::read_to_string(configuration_file_path)
        .expect(&error_message);

    serde_json_from_str(&content)
        .expect(&error_message)
}

pub fn configuration_event_initializer() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_loaded_and_parsed() {
        let configuration_path: &str = "./tests/configurations.json";
        let _configuration: Configuration = read(configuration_path);
    }
}
