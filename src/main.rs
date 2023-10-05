mod configuration;

use crate::configuration::configuration::Configuration;
use crate::configuration::event::listen;
use crate::configuration::reader::read;

fn main() {
    let configuration_path: &str = "./tests/configurations.json";
    let configuration: Configuration = read(configuration_path);

    listen(&configuration.input.path)
        .expect("TODO: panic message");

    println!("{:?}", &configuration);
}
