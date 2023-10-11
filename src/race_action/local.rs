use std::fs;
use std::path::Path;

use crate::logger;
use crate::race::uber_output::UberOutput;

pub fn copy(input: &Path, output: &UberOutput) -> Result<bool, String> {
    match output {
        UberOutput::Local { path } => match input.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(name) => {
                    let passenger_name: String = name.replace('"', "");
                    let full_destination_path: String = format!("{}/{}", path, passenger_name);

                    if let Err(error) = std::fs::copy(input, &full_destination_path) {
                        let error_message: String = format!(
                            "Cannot copy file {:?} to {}: {}",
                            input, &full_destination_path, error
                        );
                        logger::warn("local", "copy", &error_message);
                        Err(error_message)
                    } else {
                        let success_message: String = format!(
                            "Successfully copy file from {:?} to {}",
                            input, &full_destination_path
                        );
                        logger::info("local", "copy", &success_message);
                        Ok(true)
                    }
                }

                None => {
                    logger::warn("local", "copy", "Cannot get file name as string");
                    Err("Cannot get file name as string".to_string())
                }
            },

            None => {
                logger::warn("local", "copy", "Cannot get file name");
                Err("Cannot get file name".to_string())
            }
        },

        _ => {
            let error_message: String = format!("Wrong output used with the method: {:?}", output);
            logger::warn("local", "copy", &error_message);
            Err(error_message)
        }
    }
}

pub fn delete(path: &Path) -> Result<bool, String> {
    match fs::remove_file(path) {
        Ok(_) => Ok(true),

        Err(_) => {
            let error_message: String = format!("Cannot delete file: {:?}", path);
            logger::warn("local", "delete", &error_message);
            Err(error_message)
        }
    }
}
