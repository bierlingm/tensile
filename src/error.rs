use thiserror::Error;

#[derive(Error, Debug)]
pub enum TensileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Validation error: {0}")]
    #[allow(dead_code)]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("State conflict: {0}")]
    #[allow(dead_code)]
    StateConflict(String),

    #[error("Invalid state transition: {0} -> {1}")]
    InvalidStateTransition(String, String),

    #[error("Unknown error: {0}")]
    #[allow(dead_code)]
    Unknown(String),
}

pub type TensileResult<T> = Result<T, TensileError>;

impl From<ron::error::SpannedError> for TensileError {
    fn from(err: ron::error::SpannedError) -> Self {
        TensileError::Serialization(err.to_string())
    }
}

impl From<uuid::Error> for TensileError {
    fn from(err: uuid::Error) -> Self {
        TensileError::Parse(err.to_string())
    }
}

impl From<rusqlite::Error> for TensileError {
    fn from(err: rusqlite::Error) -> Self {
        TensileError::Database(err.to_string())
    }
}
