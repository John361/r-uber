use thiserror::Error;

pub mod races;
pub mod uber;
pub mod uber_input;
pub mod uber_output;

#[derive(Error, Debug)]
pub enum RaceError {
    #[error("Cannot read or parse races: {0}")]
    ReadOrParseRaces(String),
}
