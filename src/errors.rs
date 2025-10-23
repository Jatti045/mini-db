use thiserror::Error;

/// Defines our different possible errors
#[derive(Error, Debug, PartialEq)]
pub enum DbError {
    #[error("Invalid command syntax")]
    InvalidCommand,
    
    #[error("Duplicate id {0}")]
    DuplicateId(u32),

    #[error("Failed to parse input: {0}")]
    ParseError(String),
}