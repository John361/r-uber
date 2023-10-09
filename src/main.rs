use std::thread;

use crate::configuration::{ConfigKafka, Configuration};
use crate::event::event_consumer::consume_uber;
use crate::event::event_producer::listen_races;
use crate::race::races::Races;

mod configuration;
mod event;
mod logger;
mod race;
mod race_action;

fn main() {
    logger::initialize();

    let configuration: Configuration = Configuration::from_file("./tests/config").expect(""); // TODO: handle?
    let config_kafka_cloned: ConfigKafka = configuration.kafka.clone();
    let races: Races = Races::from_file(&configuration.races_path).expect(""); // TODO: handle?

    thread::spawn(move || consume_uber(&config_kafka_cloned));
    listen_races(&configuration.kafka, &races);
}
