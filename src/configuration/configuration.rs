use serde::{Serialize, Deserialize};
use serde_json::{from_str as serde_json_from_str, to_string as serde_json_to_string};

#[derive(Deserialize, Debug)]
pub struct Races {
    pub ubers: Vec<Uber>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Uber {
    pub input: UberInput,
    pub outputs: Vec<UberOutput>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UberInput {
    pub path: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UberOutput {
    pub path: String
}

impl Races {

    pub fn to_string(&self) -> String {
        serde_json_to_string(self)
            .expect("Cannot serialize races")
    }

    pub fn from_string(content: &str) -> Self {
        serde_json_from_str(content)
            .expect("Cannot deserialize races")
    }
}

impl Uber {

    pub fn to_string(&self) -> String {
        serde_json_to_string(self)
            .expect("Cannot serialize uber")
    }

    pub fn from_string(content: &str) -> Self {
        serde_json_from_str(content)
            .expect("Cannot deserialize uber")
    }
}
