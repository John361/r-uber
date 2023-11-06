use serde::{Deserialize, Serialize};
use serde_json::{from_str as serde_json_from_str, to_string as serde_json_to_string};

use crate::event::EventError;
use crate::race::uber::Uber;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventUber {
    pub uber: Uber,
    pub passenger: String,
}

impl EventUber {
    pub fn to_json_string(&self) -> Result<String, EventError> {
        let result = serde_json_to_string(self)
            .map_err(|error| EventError::ReadOrParseEvent(error.to_string()))?;

        Ok(result)
    }

    pub fn from_json_string(content: &str) -> Result<Self, EventError> {
        let result = serde_json_from_str::<EventUber>(content)
            .map_err(|error| EventError::ReadOrParseEvent(error.to_string()))?;

        Ok(result)
    }
}
