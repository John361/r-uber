use serde::{Deserialize, Serialize};
use serde_json::{from_str as serde_json_from_str, to_string as serde_json_to_string};

use crate::logger;
use crate::race::uber::Uber;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventUber {
    pub uber: Uber,
    pub passenger: String,
}

impl EventUber {
    pub fn to_json_string(&self) -> Result<String, String> {
        match serde_json_to_string(self) {
            Ok(content) => { Ok(content) }

            Err(error) => {
                let error_message = format!("Cannot convert EventUber to String: {}", error);
                logger::warn("uber_event", "to_json_string", &error_message);
                Err(error_message)
            }
        }
    }

    pub fn from_json_string(content: &str) -> Result<Self, String> {
        match serde_json_from_str::<EventUber>(content) {
            Ok(event_uber) => { Ok(event_uber) }

            Err(error) => {
                let error_message = format!("Cannot convert from string to EventUber: {}", error);
                logger::warn("uber_event", "from_json_string", &error_message);
                Err(error_message)
            }
        }
    }
}
