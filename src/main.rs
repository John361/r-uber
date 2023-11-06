use std::error::Error;
use std::thread;

use crate::argument::Argument;
use crate::configuration::{ConfigKafka, Configuration};
use crate::event::event_consumer::consume_uber;
use crate::event::event_producer::listen_races;
use crate::race::races::Races;

mod argument;
mod configuration;
mod event;
mod logger;
mod race;
mod race_action;

fn main() -> Result<(), Box<dyn Error>> {
    logger::initialize();

    let argument = Argument::build()?;
    let configuration = Configuration::from_file(&argument.config_path)?;
    let config_kafka_cloned: ConfigKafka = configuration.kafka.clone();
    let races = Races::from_file(&configuration.races_path)?;

    thread::spawn(move || consume_uber(&config_kafka_cloned));
    listen_races(&configuration.kafka, &races);

    Ok(())
}
