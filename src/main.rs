mod configuration;

use crate::configuration::configuration::Configuration;
use crate::configuration::configuration_reader::read;

fn main() {

    let configuration_path: &str = "./tests/configurations.json";
    let configuration_content: Configuration = read(configuration_path);

    println!("{:?}", &configuration_content);
}
