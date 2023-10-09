use std::fs;

use serde::Deserialize;
use serde_json::from_str as serde_json_from_str;

use crate::logger;
use crate::race::uber::Uber;

#[derive(Deserialize, Debug)]
pub struct Races {
    pub ubers: Vec<Uber>,
}

impl Races {
    pub fn from_string(content: &str) -> Result<Self, String> {
        match serde_json_from_str::<Races>(content) {
            Ok(result) => {
                logger::info("races", "from_string", "Successfully convert races string to Races");
                Ok(result)
            }

            Err(error) => {
                let error_message: String = format!("Cannot convert from string to Races: {}", error);
                logger::error("races", "from_string", &error_message);
                Err(error_message)
            }
        }
    }

    pub fn from_file(file_path: &str) -> Result<Self, String> {
        match fs::read_to_string(file_path) {
            Ok(content) => {
                match Races::from_string(&content) {
                    Ok(races) => {
                        logger::info("races", "from_file", "Successfully load races from file");
                        Ok(races)
                    }

                    Err(error) => {
                        logger::error("races", "from_file", &error);
                        Err(error)
                    }
                }
            }

            Err(error) => {
                let error_message: String = format!("Cannot read file to string: {}", error);
                logger::error("races", "from_file", &error_message);
                Err(error_message)
            }
        }
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
        let _result: Races = Races::from_file(file_path).expect("");
    }
}
