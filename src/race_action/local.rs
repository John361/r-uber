use std::fs;
use std::path::Path;

use crate::logger;
use crate::race::uber_output::UberOutput;
use crate::race_action::RaceActionError;

pub fn copy(input: &Path, output: &UberOutput) -> Result<bool, RaceActionError> {
    match output {
        UberOutput::Local { path } => {
            let file_name = input
                .file_name()
                .ok_or(RaceActionError::FileName(path.to_string()))?;
            let file_name = file_name
                .to_str()
                .ok_or(RaceActionError::FileName(path.to_string()))?;
            let passenger_name: String = file_name.replace('"', "");
            let full_destination_path: String = format!("{}/{}", path, passenger_name);

            fs::copy(input, &full_destination_path).map_err(|error| {
                logger::warn("local", "copy", &error.to_string());
                RaceActionError::Copy {
                    source_path: input.to_string_lossy().to_string(),
                    destination: full_destination_path.to_string(),
                }
            })?;

            let success_message: String = format!(
                "Successfully copy file from {:?} to {}",
                input, &full_destination_path
            );
            logger::info("local", "copy", &success_message);
            Ok(true)
        }

        _ => {
            logger::warn("local", "copy", "Wrong output used with method");
            Err(RaceActionError::WrongOutput)
        }
    }
}

pub fn delete(path: &Path) -> Result<bool, RaceActionError> {
    fs::remove_file(path).map_err(|error| {
        logger::warn("local", "delete", &error.to_string());
        RaceActionError::Delete(path.to_string_lossy().to_string())
    })?;

    Ok(true)
}
