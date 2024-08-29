use thiserror::Error;

pub type ChurResult<T> = Result<T, ChurError>;

#[derive(Debug, Error)]
pub enum ChurError {
    #[error("Failed to open file or directory - {0:?}")]
    IOError(#[from] std::io::Error),

    #[error("Failed to deserialise Manifest - {0}")]
    Manifest(String),

    #[error("Network Error - {0}")]
    Network(#[from] ureq::Error),

    #[error("Dependency Error - {0}")]
    Dependency(String),

    #[error("Archive Error - {0}")]
    Archive(#[from] archiver_rs::ArchiverError)
}

impl From<ron::error::Error> for ChurError {
    fn from(value: ron::error::Error) -> Self {
        Self::Manifest(value.to_string())
    }
}

impl From<ron::error::SpannedError> for ChurError {
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Manifest(value.to_string())
    }
}