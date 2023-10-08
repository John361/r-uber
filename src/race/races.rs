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
}
