//! Error types for database operations.
//!
//! This module defines all possible error conditions that can occur
//! during database operations, using the `thiserror` crate for
//! automatic error trait implementations.

use thiserror::Error;

/// Comprehensive error type for all database operations.
///
/// This enum covers all failure modes including:
/// - Command parsing errors
/// - Constraint violations (duplicate IDs)
/// - I/O failures
/// - Serialization/deserialization errors
#[derive(Error, Debug)]
pub enum DbError {
    /// Returned when a command has invalid syntax or format
    #[error("Invalid command syntax")]
    InvalidCommandError,
    
    /// Returned when attempting to insert a row with an ID that already exists
    #[error("Duplicate id {0}")]
    DuplicateIdError(u32),

    /// Returned when parsing input data fails
    #[error("Failed to parse input: {0}")]
    ParseError(String),

    /// Returned when file or I/O operations fail
    /// 
    /// This wraps standard library I/O errors with automatic conversion
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Returned when JSON serialization or deserialization fails
    ///
    /// This wraps serde_json errors with automatic conversion
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error)
}
