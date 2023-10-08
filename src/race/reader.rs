use std::fs;

use crate::race::races::Races;

pub fn read(races_file_path: &str) -> Races {
    let error_message: String = format!("Cannot read races list file from path {0}",
                                        races_file_path);

    let content: String = fs::read_to_string(races_file_path)
        .expect(&error_message);

    Races::from_string(&content)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn races_loaded_and_parsed() {
        let races_path: &str = "./tests/races.json";
        let _races: Races = read(races_path);
    }
}
