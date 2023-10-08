use std::fs;

use serde::Deserialize;
use serde_json::from_str as serde_json_from_str;

use crate::race::uber::Uber;

#[derive(Deserialize, Debug)]
pub struct Races {
    pub ubers: Vec<Uber>
}

impl Races {

    pub fn from_string(content: &str) -> Self {
        serde_json_from_str(content)
            .expect("Cannot deserialize races")
    }

    pub fn from_file(file_path: &str) -> Self {
        let error_message: String = format!("Cannot read races list file from path {0}",
                                            file_path);

        let content: String = fs::read_to_string(file_path)
            .expect(&error_message);

        Races::from_string(&content)
    }

    pub fn has_uber_with_same_input_path(&self, other_path: &str) -> Option<&Uber> {
        self.ubers.iter()
            .filter(|uber| uber.input.is_same_path(other_path))
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn races_loaded_and_parsed() {
        let file_path: &str = "./tests/races.json";
        let _result: Races = Races::from_file(file_path);
    }
}
