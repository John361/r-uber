use kafka::consumer::{Consumer, FetchOffset};

use crate::race::uber::Uber;

pub fn consume_uber() {
    let hosts: Vec<String> = vec!["localhost:9092".to_owned()];

    let mut consumer: Consumer = Consumer::from_hosts(hosts)
            .with_topic("uber-race".to_owned())
            .with_fallback_offset(FetchOffset::Latest)
            .create()
            .expect("Cannot create uber consumer");

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let content: &str = std::str::from_utf8(m.value).unwrap();
                let uber: Uber = Uber::from_string(content);
                println!("{:?}", uber);
            }

            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap_or_default();
    }
}
