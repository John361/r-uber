use serde::{Deserialize, Serialize};
use serde_json::{from_str as serde_json_from_str, to_string as serde_json_to_string};

use crate::race::uber_input::UberInput;
use crate::race::uber_output::UberOutput;

#[derive(Serialize, Deserialize, Debug)]
pub struct Uber {
    pub input: UberInput,
    pub outputs: Vec<UberOutput>,
}

impl Uber {
    pub fn to_json_string(&self) -> String {
        serde_json_to_string(self).expect("Cannot serialize uber")
    }

    pub fn from_json_string(content: &str) -> Self {
        serde_json_from_str(content).expect("Cannot deserialize uber")
    }
}
