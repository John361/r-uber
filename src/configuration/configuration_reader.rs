use std::fs;

use serde_json::{from_str as serde_json_from_str};
use crate::configuration::configuration::Configuration;

pub fn read(configuration_file_path: &str) -> Configuration {
    let error_message: String = format!("Cannot read configurations file from path {0}",
                                        configuration_file_path);

    let content: String = fs::read_to_string(configuration_file_path)
        .expect(&error_message);

    serde_json_from_str(&content)
        .expect(&error_message)
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
