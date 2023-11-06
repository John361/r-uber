use thiserror::Error;

pub mod local;
pub mod sftp;

#[derive(Error, Debug)]
pub enum RaceActionError {
    #[error("Cannot get file name: {0}")]
    FileName(String),
    #[error("Cannot copy file {source_path:?} to {destination:?}")]
    Copy {
        source_path: String,
        destination: String,
    },
    #[error("Cannot delete file: {0}")]
    Delete(String),
    #[error("Wrong output used with method")]
    WrongOutput,
    #[error("Error during session usage: {0}")]
    SessionUsage(String)
}
