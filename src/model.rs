//! Data model definitions for database rows.
//!
//! This module defines the core data structures stored in the database.
//! All models implement Serialize/Deserialize for JSON persistence.

use serde::{Serialize, Deserialize};

/// Represents a single row in the database.
///
/// Each row contains:
/// - A unique ID (primary key)
/// - A name field (string)
/// - An age field (unsigned 8-bit integer, 0-255)
///
/// # Examples
///
/// ```
/// use mini_db::model::Row;
///
/// let row = Row::new(1, "Alice".to_string(), 30);
/// assert_eq!(row.id, 1);
/// assert_eq!(row.name, "Alice");
/// assert_eq!(row.age, 30);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row {
    /// Unique identifier for the row (primary key)
    pub id: u32,
    /// Name field
    pub name: String,
    /// Age field (0-255)
    pub age: u8,
}

impl Row {
    /// Creates a new row with the given values.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the row
    /// * `name` - Name value for the row
    /// * `age` - Age value (0-255)
    ///
    /// # Returns
    ///
    /// A new `Row` instance with the specified values.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_db::model::Row;
    ///
    /// let row = Row::new(42, "Bob".to_string(), 25);
    /// ```
    pub fn new(id: u32, name: String, age: u8) -> Self {
        Self {
            id,
            name,
            age,
        }
    }
}