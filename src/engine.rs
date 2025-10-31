//! Database engine module providing the core database functionality.
//!
//! This module contains the `Database` struct which manages:
//! - In-memory row storage
//! - ID-based indexing for fast lookups
//! - Persistence through an append-only log
//! - CRUD operations (Create, Read, Update, Delete)

use fs_err::File;

use crate::parser;
use crate::{index::IdIndex, model::Row};
use crate::errors::DbError;
use crate::storage::Storage;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

/// The main database structure that manages all database operations.
///
/// # Components
///
/// - `rows`: In-memory storage of all database rows
/// - `index`: Hash-based index mapping IDs to row positions for O(1) lookups
/// - `storage`: Persistence layer handling the append-only log
pub struct Database {
    /// In-memory vector of all rows currently in the database
    rows: Vec<Row>,
    /// Index mapping row IDs to their positions in the rows vector
    index: IdIndex,
    /// Storage backend for persisting operations to disk
    storage: Storage,
}

impl Database {
    /// Creates a new database instance or loads an existing one from the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path where the database log is stored
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the database instance or a `DbError` if:
    /// - The file cannot be created or opened
    /// - The existing log file is corrupted
    /// - There are I/O errors during log replay
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_db::engine::Database;
    ///
    /// let db = Database::new("data.json")?;
    /// # Ok::<(), mini_db::errors::DbError>(())
    /// ```
    pub fn new(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let storage = Storage::new(path)?;
        let rows = storage.load_all()?;
        let index = IdIndex::rebuild(&rows);

        Ok(Self {
            rows,
            index,
            storage,
        })
    }

    /// Inserts a new row into the database.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the row (must not already exist)
    /// * `name` - Name field for the row
    /// * `age` - Age field for the row
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or a `DbError` if:
    /// - The ID already exists (`DuplicateIdError`)
    /// - There are I/O errors writing to the log
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mini_db::engine::Database;
    /// # let mut db = Database::new("data.json")?;
    /// db.insert(1, "Alice".to_string(), 30)?;
    /// # Ok::<(), mini_db::errors::DbError>(())
    /// ```
    pub fn insert(&mut self, id: u32, name: String, age: u8) -> Result<(), DbError> {
        // Check for duplicate IDs to maintain uniqueness constraint
        if self.rows.iter().any(|r| r.id == id) {
            return Err(DbError::DuplicateIdError(id));
        }

        let newly_created_row = Row::new(id, name, age);
        self.storage.append_entry(&newly_created_row)?;
        self.rows.push(newly_created_row);
        self.index.insert(id, self.rows.len() - 1)?;

        Ok(())
    }

    /// Executes a batch of commands from a text file.
    ///
    /// Each line in the file should contain a valid database command.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file containing batch commands
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or a `DbError` if:
    /// - The file does not exist
    /// - There are I/O errors reading the file
    /// - Any command in the batch fails
    pub fn exec_batch(&mut self, path: PathBuf) -> Result<(), DbError> {
        if !path.exists() {
            return Err(DbError::InvalidCommandError);
        }

        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            let line = line?;
            parser::handle_command(&line, self);
        }

        Ok(())
    }

    /// Returns a reference to all rows in the database.
    ///
    /// # Returns
    ///
    /// A reference to the vector containing all rows in insertion order
    /// (accounting for deletions).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mini_db::engine::Database;
    /// # let db = Database::new("data.json")?;
    /// let all_rows = db.select_all();
    /// println!("Total rows: {}", all_rows.len());
    /// # Ok::<(), mini_db::errors::DbError>(())
    /// ```
    pub fn select_all(&self) -> &Vec<Row> {
        &self.rows
    }

    /// Resets the database by clearing all data and truncating the log file.
    ///
    /// **Warning**: This operation is irreversible and will delete all data.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or a `DbError` if there are I/O errors.
    pub fn reset_db(&mut self) -> Result<(), DbError> {
        self.rows.clear();
        self.index.clear();

        let path = &self.storage.path;
        // Truncate the file by recreating it
        File::create(path)?;

        Ok(())
    }

    /// Safely shuts down the database by flushing all pending writes to disk.
    ///
    /// This ensures data durability by syncing the log file before the database
    /// is dropped or the program exits.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or a `DbError` if the flush operation fails.
    pub fn shutdown(&mut self) -> Result<(), DbError> {
        self.storage.flush()?;

        Ok(())
    }

    /// Deletes a row by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the row to delete
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the row was found and deleted, `Ok(false)` if the
    /// row was not found, or a `DbError` if there are I/O errors.
    ///
    /// # Note
    ///
    /// After deletion, the index is rebuilt to maintain consistency of row positions.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mini_db::engine::Database;
    /// # let mut db = Database::new("data.json")?;
    /// # db.insert(1, "Alice".to_string(), 30)?;
    /// let deleted = db.delete_by_id(1)?;
    /// assert!(deleted);
    /// # Ok::<(), mini_db::errors::DbError>(())
    /// ```
    pub fn delete_by_id(&mut self, id: u32) -> Result<bool, DbError> {
        if let Some(pos) = self.index.get(id) {
            self.index.remove(id);
            self.storage.append_delete(id)?;
            self.rows.remove(pos);

            // Rebuild index since positions have shifted after removal
            let index = IdIndex::rebuild(&self.rows);
            self.index = index;

            return Ok(true);
        }
        Ok(false)
    }

    /// Retrieves a row by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the row to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(Row))` if found, `Ok(None)` if not found,
    /// or a `DbError` if there are errors.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mini_db::engine::Database;
    /// # let db = Database::new("data.json")?;
    /// if let Some(row) = db.select_by_id(1)? {
    ///     println!("Found: {} (age {})", row.name, row.age);
    /// }
    /// # Ok::<(), mini_db::errors::DbError>(())
    /// ```
    pub fn select_by_id(&self, id: u32) -> Result<Option<Row>, DbError> {
        if let Some(pos) = self.index.get(id) {
            Ok(Some(self.rows[pos].clone()))
        } else {
            Ok(None)
        }
    }

    /// Gets the internal index position for a given ID.
    ///
    /// This method is primarily used for testing to verify index correctness.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID to look up in the index
    ///
    /// # Returns
    ///
    /// Returns `Some(position)` if the ID exists in the index, `None` otherwise.
    pub fn get_index_position(&self, id: u32) -> Option<usize> {
        self.index.get(id)
    }
}