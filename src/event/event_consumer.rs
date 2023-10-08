use kafka::consumer::{Consumer, FetchOffset};

use crate::configuration::configuration::Configuration;

pub fn consume() {
    let hosts: Vec<String> = vec!["localhost:9092".to_owned()];

    let mut consumer: Consumer = Consumer::from_hosts(hosts)
            .with_topic("topic-name".to_owned())
            .with_fallback_offset(FetchOffset::Latest)
            .create()
            .expect("Cannot create consumer");

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let content: &str = std::str::from_utf8(m.value).unwrap();
                let configuration: Configuration = Configuration::from_string(content);
                println!("{:?}", configuration);
            }

            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap_or_default();
    }
}
