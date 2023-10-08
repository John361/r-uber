use std::thread;

use crate::configuration::configuration::Configuration;
use crate::configuration::reader::read;
use crate::event::event_consumer::consume;
use crate::event::event_producer::listen;

mod configuration;
mod event;

fn main() {
    let configuration_path: &str = "./tests/configurations.json";
    let configuration: Configuration = read(configuration_path);

    thread::spawn(|| {
        consume()
    });

    listen(&configuration)
        .expect("TODO: panic message");
}
