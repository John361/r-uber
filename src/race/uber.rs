use serde::{Deserialize, Serialize};

use crate::logger;
use crate::race::uber_input::UberInput;
use crate::race::uber_output::UberOutput;
use crate::race_action::local;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Uber {
    pub input: UberInput,
    pub outputs: Vec<UberOutput>,
}

impl Uber {
    pub fn kick_passenger(&self, passenger: &str) {
        if let Err(error) = local::delete(passenger.as_ref()) {
            let error_message = format!("Error occurred when trying to kick passenger: {}", error);
            logger::warn("uber", "kick_passenger", &error_message);
        } else {
            let success_message = format!("Successfully kick passenger {}", passenger);
            logger::info("uber", "kick_passenger", &success_message);
        }
    }
}
