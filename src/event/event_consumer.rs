use kafka::consumer::{Consumer, FetchOffset};

use crate::configuration::ConfigKafka;
use crate::event::event_uber::EventUber;

pub fn consume_uber(kafka_config: &ConfigKafka) {
    let mut consumer: Consumer = Consumer::from_hosts(kafka_config.hosts.to_owned())
        .with_topic(kafka_config.topic.to_owned())
        .with_fallback_offset(FetchOffset::Latest)
        .create()
        .expect("Cannot create uber consumer");

    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                let content: &str = std::str::from_utf8(m.value).unwrap();
                let message: EventUber = EventUber::from_json_string(content);

                for uber_output in &message.uber.outputs {
                    uber_output.take_passenger_and_drive_to(&message.uber, &message.passenger);
                }
            }

            consumer.consume_messageset(ms).unwrap();
        }

        consumer.commit_consumed().unwrap_or_default();
    }
}
