use serde::{Deserialize, Serialize};

use crate::race::uber_input::UberInput;
use crate::race::uber_output::UberOutput;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Uber {
    pub input: UberInput,
    pub outputs: Vec<UberOutput>,
}
