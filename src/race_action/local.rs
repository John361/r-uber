use std::path::Path;

use crate::race::uber_output::UberOutput;

pub fn copy(input: &Path, output: &UberOutput) -> std::io::Result<u64> {
    match output {
        UberOutput::Local { path } => {
            let passenger_name = input.file_name().unwrap()
                .to_str().unwrap()
                .replace('"', "");
            let full_destination_path = format!("{}/{}", path, passenger_name);

            std::fs::copy(input, full_destination_path)
        }

        _ => { Ok(0) }
    }
}
