use kafka::consumer::{Consumer, FetchOffset};

use crate::configuration::ConfigKafka;
use crate::event::event_uber::EventUber;
use crate::logger;

pub fn consume_uber(kafka_config: &ConfigKafka) {
    let client_consumer = Consumer::from_hosts(kafka_config.hosts.to_owned())
        .with_topic(kafka_config.topic.to_owned())
        .with_fallback_offset(FetchOffset::Latest)
        .create();

    match client_consumer {
        Ok(mut consumer) => loop {
            for ms in consumer.poll().unwrap().iter() {
                for m in ms.messages() {
                    match std::str::from_utf8(m.value) {
                        Ok(content) => match EventUber::from_json_string(content) {
                            Ok(message) => {
                                for uber_output in &message.uber.outputs {
                                    uber_output.take_passenger_and_drive_to(
                                        &message.uber,
                                        &message.passenger,
                                    );
                                }

                                message.uber.kick_passenger(&message.passenger);
                            }

                            Err(error) => {
                                let error_message: String =
                                    format!("Cannot convert uber event message: {}", error);
                                logger::error("event_consumer", "consume_uber", &error_message);
                            }
                        },

                        Err(error) => {
                            let error_message: String =
                                format!("Cannot read uber event message: {}", error);
                            logger::error("event_consumer", "consume_uber", &error_message);
                        }
                    }
                }

                consumer.consume_messageset(ms).unwrap();
            }

            consumer.commit_consumed().unwrap_or_default();
        },

        Err(error) => {
            let error_message: String = format!("Cannot create uber consumer: {}", error);
            logger::error("event_consumer", "consume_uber", &error_message);
        }
    }
}
