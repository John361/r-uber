use crate::race::uber_output::UberOutput;

pub fn copy(input: &str, output: &UberOutput) -> std::io::Result<u64> {
    match output {
        UberOutput::Local { path } => {
            std::fs::copy(input, path)
        }

        _ => { Ok(0) }
    }
}
