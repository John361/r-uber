use std::thread;
use std::time::Duration;

use kafka::client::{KafkaClient, RequiredAcks};
use kafka::producer::{Producer, Record};
use notify::{Config, EventKind, INotifyWatcher, RecommendedWatcher, RecursiveMode, Result, Watcher};
use notify::event::AccessKind::Close;
use notify::event::AccessMode::Write;

use crate::configuration::configuration::ConfigKafka;
use crate::race::races::Races;
use crate::race::uber::Uber;

pub fn listen_races(kafka_config: &ConfigKafka, races: &Races) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: INotifyWatcher = RecommendedWatcher::new(tx, Config::default())?;

    println!("Listening {}", races.ubers[0].input.path);
    watcher.watch(races.ubers[0].input.path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == EventKind::Access(Close(Write)) {
                    produce_message(kafka_config, &races.ubers[0]).expect("TODO: panic message");
                }
            },
            Err(error) => {
                println!("watch error: {:?}", error);
            }
        }
    }

    Ok(())
}

fn produce_message(kafka_config: &ConfigKafka, uber: &Uber) -> Result<()> {
    // https://github.com/kafka-rust/kafka-rust/issues/135#issuecomment-259823379

    let mut client: KafkaClient = KafkaClient::new(kafka_config.hosts.to_owned());
    let mut attempt: u8 = 0;

    loop {
        attempt += 1;
        let _ = client.load_metadata(&[kafka_config.topic.to_owned()])
            .expect("Cannot load metadata");

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
        value: uber.to_string()
    }).expect("Cannot send record");

    Ok(())
}
