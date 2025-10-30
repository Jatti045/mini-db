use fs_err::File;

use crate::{model::Row, storage::LogEntry};
use crate::errors::DbError;
use crate::storage::Storage;
use std::path::{Path};

/// Defines our database
pub struct Database {
    rows: Vec<Row>,
    storage: Storage,
}

impl Database {
    /// Database constructor
    pub fn new(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let storage = Storage::new(path)?;
        let rows = storage.load_all()?;

        Ok(Self {
            rows,
            storage,
        })
    }

    /// Insert new row into database by id, name, and age
    pub fn insert(&mut self, id: u32, name: String, age: u8) -> Result<(), DbError> {

        // Iterates over database rows and verifies whether id of row to be inserted exists in db
        if self.rows.iter().any(|r| r.id == id) {
            return Err(DbError::DuplicateIdError(id));
        }

        let newly_created_row = Row::new(id, name, age);
        self.storage.append_entry(&newly_created_row)?;
        self.rows.push(newly_created_row);

        Ok(())
    }

    /// Returns all rows from the database
    pub fn select_all(&self) -> &Vec<Row> {
        &self.rows
    }

    /// resets our db
    pub fn reset_db(&mut self) -> Result<(), DbError> {
        self.rows.clear();

        let path = &self.storage.path;

        // Created new file overwriting current one
        File::create(path)?;

        Ok(())
    }

    /// Safely exists database by ensuring all writes are flushed and synched to disk
    pub fn shutdown(&mut self) -> Result<(), DbError> {
        self.storage.flush()?;

        Ok(())
    }

    /// Deletes row by id
    pub fn delete_by_id(&mut self, id: u32) -> Result<bool, DbError> {
        if let Some(pos) = self.rows.iter().position(|row| row.id == id) {
            self.storage.append_delete(id)?;
            self.rows.remove(pos);
            return Ok(true)
        }
        Ok(false)
    }

    /// Selects row by id
    pub fn select_by_id(&self, id: u32) -> Result<Option<Row>, DbError> {
        let rows = &self.rows;

        Ok(rows.iter().find(|row| row.id == id).cloned())
    }
}