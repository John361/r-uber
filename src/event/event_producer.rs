use std::thread;
use std::time::Duration;

use kafka::client::{KafkaClient, RequiredAcks};
use kafka::producer::{Producer, Record};
use notify::{Config, EventKind, INotifyWatcher, RecommendedWatcher, RecursiveMode, Result, Watcher};
use notify::event::AccessKind::Close;
use notify::event::AccessMode::Write;

use crate::configuration::ConfigKafka;
use crate::race::races::Races;
use crate::race::uber::Uber;

pub fn listen_races(kafka_config: &ConfigKafka, races: &Races) {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: INotifyWatcher = RecommendedWatcher::new(tx, Config::default())
        .expect("Cannot create watcher");

    for uber in &races.ubers {
        watcher.watch(uber.input.path.as_ref(), RecursiveMode::Recursive).unwrap();
    }

    loop {
        match rx.recv() {
            Ok(event_result) => {
                if let Ok(event) = event_result {
                    if event.kind == EventKind::Access(Close(Write)) {
                        let path = event.paths[0].to_str();

                        if let Some(uber) = races.has_uber_with_same_input_path(path.unwrap()) {
                            produce_message(kafka_config, uber)
                                .expect("TODO: panic message");
                        }
                    }
                }
            }

            Err(e) => println!("Cannot get event: {:?}", e),
        }
    }
}

fn produce_message(kafka_config: &ConfigKafka, uber: &Uber) -> Result<()> {
    // https://github.com/kafka-rust/kafka-rust/issues/135#issuecomment-259823379

    let mut client: KafkaClient = KafkaClient::new(kafka_config.hosts.to_owned());
    let mut attempt: u8 = 0;

    loop {
        attempt += 1;

        {
            client.load_metadata(&[kafka_config.topic.to_owned()])
                .expect("Cannot load metadata");
        }

        if client.topics().partitions(&kafka_config.topic)
            .map(|p| p.len())
            .unwrap_or(0) > 0 {
            break;
        } else if attempt > 2 { // try up to 3 times
            // return some error
            // return Err(KafkaError::Kafka(KafkaCode::UnknownTopicOrPartition));
        }

        thread::sleep(Duration::from_secs(1));
    }

    let mut producer: Producer = Producer::from_client(client)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .expect("Cannot create uber producer");

    producer.send(&Record {
        topic: &kafka_config.topic,
        partition: -1,
        key: (),
        value: uber.to_json_string()
    }).expect("Cannot send record");

    Ok(())
}
