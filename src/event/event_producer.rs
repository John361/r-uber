use std::thread;
use std::time::Duration;

use kafka::client::{KafkaClient, RequiredAcks};
use kafka::producer::{Producer, Record};
use notify::event::AccessKind::Close;
use notify::event::AccessMode::Write;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::configuration::ConfigKafka;
use crate::event::event_uber::EventUber;
use crate::event::EventError;
use crate::logger;
use crate::race::races::Races;

pub fn listen_races(kafka_config: &ConfigKafka, races: &Races) {
    let (tx, rx) = std::sync::mpsc::channel();
    let recommended_watcher = RecommendedWatcher::new(tx, Config::default());

    match recommended_watcher {
        Ok(mut watcher) => {
            for uber in &races.ubers {
                if let Err(error) =
                    watcher.watch(uber.input.path.as_ref(), RecursiveMode::Recursive)
                {
                    let error_message: String = format!("Cannot watch: {}", error);
                    logger::error("event_producer", "listen_races", &error_message);
                }
            }

            loop {
                match rx.recv() {
                    Ok(event_result) => {
                        if let Ok(event) = event_result {
                            if event.kind == EventKind::Access(Close(Write)) {
                                if let Some(path) = event.paths[0].to_str() {
                                    if let Some(uber) = races.has_uber_with_same_input_path(path) {
                                        let event_message: EventUber = EventUber {
                                            uber: uber.clone(),
                                            passenger: path.to_string(),
                                        };

                                        match produce_message(kafka_config, event_message.clone()) {
                                            Ok(_) => {
                                                let success_message: String = format!(
                                                    "Successfully produce event for passenger {}",
                                                    &event_message.passenger
                                                );
                                                logger::info(
                                                    "event_producer",
                                                    "listen_races",
                                                    &success_message,
                                                );
                                            }

                                            Err(error) => {
                                                let error_message: String =
                                                    format!("Cannot produce event: {}", error);
                                                logger::error(
                                                    "event_producer",
                                                    "listen_races",
                                                    &error_message,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Err(error) => {
                        let error_message: String = format!("Cannot get event: {}", error);
                        logger::error("event_producer", "listen_races", &error_message);
                    }
                }
            }
        }

        Err(error) => {
            let error_message: String = format!("Cannot create watcher: {}", error);
            logger::error("event_producer", "listen_races", &error_message);
        }
    }
}

fn produce_message(kafka_config: &ConfigKafka, event: EventUber) -> Result<bool, EventError> {
    // https://github.com/kafka-rust/kafka-rust/issues/135#issuecomment-259823379

    let mut client: KafkaClient = KafkaClient::new(kafka_config.hosts.to_owned());
    let mut attempt: u8 = 0;

    loop {
        attempt += 1;

        {
            client
                .load_metadata(&[kafka_config.topic.to_owned()])
                .map_err(|error| EventError::Producer(error.to_string()))
                .expect("Cannot load metadata");
        }

        if client
            .topics()
            .partitions(&kafka_config.topic)
            .map(|p| p.len())
            .unwrap_or(0)
            > 0
        {
            break;
        } else if attempt > 2 {
            // try up to 3 times
            logger::warn(
                "event_producer",
                "produce_message",
                "Kafka error with client",
            );
            return Err(EventError::Producer("Kafka error with client".to_string()));
        }

        thread::sleep(Duration::from_secs(1));
    }

    let mut producer = Producer::from_client(client)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .map_err(|error| EventError::Producer(error.to_string()))?;

    let content = event
        .to_json_string()
        .map_err(|error| EventError::Producer(error.to_string()))?;

    producer
        .send(&Record {
            topic: &kafka_config.topic,
            partition: -1,
            key: (),
            value: content,
        })
        .map_err(|error| EventError::Producer(error.to_string()))?;

    Ok(true)
}
