use thiserror::Error;

pub mod event_consumer;
pub mod event_producer;
pub mod event_uber;

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Cannot read or parse event: {0}")]
    ReadOrParseEvent(String),
    #[error("Cannot produce event: {0}")]
    Producer(String),
}
