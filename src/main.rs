use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::{from_str as serde_json_from_str};

fn main() {
    let configuration_path: &str = "./tests/configurations.json";
    let configuration_content: Configuration = read_configuration_file(configuration_path);

    println!("{:?}", &configuration_content);
}

fn read_configuration_file(configuration_file_path: &str) -> Configuration {
    let error_message: String = format!("Cannot read configuration file from path {0}",
                                configuration_file_path);

    let content: String = fs::read_to_string(configuration_file_path)
        .expect(&error_message);

    serde_json_from_str(&content)
        .expect(&error_message)
}

#[derive(Serialize, Deserialize, Debug)]
struct Configuration {
    hello: String,
    foo: String
}
