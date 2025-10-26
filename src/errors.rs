use thiserror::Error;

/// Defines our different possible errors
#[derive(Error, Debug)]
pub enum DbError {
    #[error("Invalid command syntax")]
    InvalidCommandError,
    
    #[error("Duplicate id {0}")]
    DuplicateIdError(u32),

    #[error("Failed to parse input: {0}")]
    ParseError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error)
}
