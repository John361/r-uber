use std::fs;

use serde::Deserialize;
use serde_json::from_str as serde_json_from_str;

use crate::logger;
use crate::race::RaceError;
use crate::race::uber::Uber;

#[derive(Deserialize, Debug)]
pub struct Races {
    pub ubers: Vec<Uber>,
}

impl Races {
    pub fn from_string(content: &str) -> Result<Self, RaceError> {
        let result = serde_json_from_str::<Races>(content)
            .map_err(|error| RaceError::ReadOrParseRaces(error.to_string()))?;

        logger::info(
            "races",
            "from_string",
            "Successfully convert races string to Races",
        );
        Ok(result)
    }

    pub fn from_file(file_path: &str) -> Result<Self, RaceError> {
        let content = fs::read_to_string(file_path)
            .map_err(|error| {
                logger::error("races", "from_file", "Cannot read races");
                RaceError::ReadOrParseRaces(error.to_string())
            })?;

        let races = Races::from_string(&content)?;
        logger::info("races", "from_file", "Successfully load races from file");
        Ok(races)
    }

    pub fn has_uber_with_same_input_path(&self, other_path: &str) -> Option<&Uber> {
        self.ubers
            .iter()
            .find(|uber| uber.input.is_same_path(other_path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn races_loaded_and_parsed() {
        let file_path: &str = "./tests/races.json";
        let _result: Races = Races::from_file(file_path).unwrap();
    }
}
