//! Index module providing fast ID-based lookups.
//!
//! This module implements a hash-based index that maps row IDs to their
//! positions in the database's row vector, enabling O(1) lookups by ID.

use std::collections::HashMap;
use crate::model::Row;
use crate::errors::DbError;

/// A hash-based index mapping row IDs to their positions in the database.
///
/// The index provides O(1) lookups for retrieving rows by their unique ID.
/// It must be kept in sync with the actual row storage, and is typically
/// rebuilt after operations that change row positions (like deletions).
pub struct IdIndex {
    /// Maps row ID -> position in the rows vector
    row_map: HashMap<u32, usize>
}

impl IdIndex {
    /// Creates a new, empty index.
    pub fn new() -> Self {
        IdIndex {
            row_map: HashMap::new()
        }
    }

    /// Inserts a new ID-to-position mapping into the index.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique row ID
    /// * `position` - The position of the row in the storage vector
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or `DuplicateIdError` if the ID already exists.
    pub fn insert(&mut self, id: u32, position: usize) -> Result<(), DbError> {
        if self.row_map.contains_key(&id) {
            return Err(DbError::DuplicateIdError(id));
        } else {
            self.row_map.insert(id, position);
            return Ok(());
        }
    }

    /// Removes an ID from the index.
    ///
    /// This is typically called when a row is deleted. Note that this does not
    /// update positions of other entries - a full rebuild may be needed.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID to remove from the index
    pub fn remove(&mut self, id: u32)  {
        self.row_map.remove(&id);
    }

    /// Retrieves the position of a row by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID to look up
    ///
    /// # Returns
    ///
    /// Returns `Some(position)` if the ID exists, `None` otherwise.
    pub fn get(&self, id: u32) -> Option<usize> {
        self.row_map.get(&id).copied()
    }

    /// Clears all entries from the index.
    ///
    /// This is typically used when resetting the database.
    pub fn clear(&mut self) {
        self.row_map.clear();
    }

    /// Rebuilds the index from a vector of rows.
    ///
    /// This creates a fresh index by scanning through all rows and mapping
    /// their IDs to their current positions. This is necessary after operations
    /// that change row positions (like deletions) or when loading from disk.
    ///
    /// # Arguments
    ///
    /// * `rows` - A reference to the vector of rows to index
    ///
    /// # Returns
    ///
    /// A new `IdIndex` containing mappings for all rows.
    pub fn rebuild(rows: &Vec<Row>) -> Self {
        let mut row_map: HashMap<u32, usize> = HashMap::new();

        for (index, row) in rows.iter().enumerate() {
            row_map.insert(row.id, index);
        }

        IdIndex {
            row_map
        }
    }
    
     
}