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

fn main() {
    logger::initialize();

    match Argument::build() {
        Ok(argument) => match Configuration::from_file(&argument.config_path) {
            Ok(configuration) => {
                let config_kafka_cloned: ConfigKafka = configuration.kafka.clone();

                match Races::from_file(&configuration.races_path) {
                    Ok(races) => {
                        thread::spawn(move || consume_uber(&config_kafka_cloned));
                        listen_races(&configuration.kafka, &races);
                    }

                    Err(error) => {
                        let error_message: String = format!("Cannot read races file: {}", error);
                        logger::error("main", "main", &error_message);
                        panic!("{}", error_message)
                    }
                }
            }

            Err(error) => {
                let error_message: String = format!("Cannot read configuration file: {}", error);
                logger::error("main", "main", &error_message);
                panic!("{}", error_message)
            }
        },

        Err(error) => {
            let error_message: String = format!("Cannot read argument: {}", error);
            logger::error("main", "main", &error_message);
            panic!("{}", error_message)
        }
    }
}
