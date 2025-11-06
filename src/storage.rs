//! Storage module for append-only log persistence.
//!
//! This module handles all disk I/O operations for the database, implementing
//! an append-only log structure for durability and crash recovery.
//!
//! ## Log Structure
//!
//! The log file contains JSON-encoded entries, one per line:
//! - Insert operations: Store the full row data with a timestamp
//! - Delete operations: Store only the row ID to be deleted
//!
//! On startup, the log is replayed to reconstruct the database state.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use serde::{Serialize, Deserialize};
use chrono::Utc;

use crate::model::Row;
use crate::errors::DbError;

/// Represents a single entry in the append-only log.
///
/// Each log entry is serialized as JSON and written to a new line.
#[derive(Serialize, Deserialize, Debug)]
pub enum LogEntry {
    /// Represents an insert operation
    Insert {
        /// The row that was inserted
        row: Row,
        /// Unix timestamp when the insert occurred
        timestamp: i64
    },
    /// Represents a delete operation
    Delete {
        /// The ID of the row that was deleted
        id: u32,
    }
}

/// Manages persistent storage using an append-only log.
///
/// The storage layer provides:
/// - Atomic write operations (each entry is a single line)
/// - Crash recovery through log replay
/// - Sequential access for efficient bulk loading
pub struct Storage {
    /// Path to the log file on disk
    pub path: PathBuf,
    /// File handle for append operations
    pub file: File
}

impl Storage {
    /// Creates a new storage instance, opening or creating the log file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the log file
    ///
    /// # Returns
    ///
    /// Returns a `Storage` instance ready for append operations,
    /// or a `DbError` if the file cannot be opened/created.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use mini_db::storage::Storage;
    ///
    /// let storage = Storage::new("mini_db.log")?;
    /// # Ok::<(), mini_db::errors::DbError>(())
    /// ```
    pub fn new(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let dir_path = PathBuf::from("data");
        let path = dir_path.join(path.as_ref());

        // Open file in append mode, creating it if it doesn't exist
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)?;

        Ok(Storage {
            path,
            file
        })
    }

    /// Appends an insert operation to the log.
    ///
    /// The row is serialized to JSON along with a timestamp and written
    /// as a single line to the log file.
    ///
    /// # Arguments
    ///
    /// * `row` - The row to insert
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or a `DbError` if serialization or
    /// writing fails.
    pub fn append_entry(&mut self, row: &Row) -> Result<(), DbError> {
        let log_entry = LogEntry::Insert {
            row: row.clone(),
            timestamp: Utc::now().timestamp(),
        };

        // Serialize to JSON and write as a single line
        let json = serde_json::to_string(&log_entry)?;
        writeln!(self.file, "{}", json)?;    

        Ok(())
    } 

    /// Appends a delete operation to the log.
    ///
    /// Only the ID is stored in the log; the actual row removal happens
    /// during replay.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the row to delete
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or a `DbError` if serialization or
    /// writing fails.
    pub fn append_delete(&mut self, id: u32) -> Result<(), DbError> {
        let log_entry = LogEntry::Delete { id };

        // Serialize to JSON and write as a single line
        let json = serde_json::to_string(&log_entry)?;
        writeln!(self.file, "{}", json)?;

        Ok(())
    }

    /// Loads and replays all operations from the log file.
    ///
    /// This method reads the entire log file and reconstructs the database
    /// state by applying each operation in order:
    /// - Insert operations add rows to the result vector
    /// - Delete operations remove rows with matching IDs
    ///
    /// # Returns
    ///
    /// Returns a vector of all rows after replaying all operations,
    /// or a `DbError` if the file cannot be read.
    ///
    /// # Error Handling
    ///
    /// The method attempts to be resilient to corrupted entries:
    /// - Malformed lines are logged as warnings and skipped
    /// - Incomplete final lines (from crashes) are detected and skipped
    pub fn load_all(&self) -> Result<Vec<Row>, DbError> {
        let path = &self.path;

        if !path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut rows = Vec::new();

        // Loops over each line in file
        for (line_num, line_res) in reader.lines().enumerate() {
            let line = match line_res {
                Ok(l) => l.trim().to_string(),
                Err(e) => {
                    eprintln!("Warning: failed to read line {}: {}", line_num + 1, e);
                    continue;
                }
            };

            if line.is_empty() {
                continue;
            }
            
            // Deserialize each line and append to row
            match serde_json::from_str(&line) {
                Ok(LogEntry::Insert {row, ..}) => rows.push(row),
                Ok(LogEntry:: Delete { id }) => rows.retain(|r| r.id != id),
                Err(e) => {
                    eprintln!("Warning: could not parse line {}: {}", line_num + 1, e);
                    
                    if line_num == rows.len() {
                        eprintln!("Skipping possibly incomplete last line.");
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }

        Ok(rows)
    }

    /// Ensures all pending writes are flushed and synced to disk.
    ///
    /// This method performs a two-phase flush:
    /// 1. Flushes the file's internal buffer
    /// 2. Syncs all data to physical storage
    ///
    /// This guarantees durability - after this call returns successfully,
    /// all previous writes are guaranteed to survive a crash or power loss.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success or a `DbError` if flushing fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use mini_db::storage::Storage;
    /// # let mut storage = Storage::new("mini_db.log")?;
    /// // After important operations, ensure durability
    /// storage.flush()?;
    /// # Ok::<(), mini_db::errors::DbError>(())
    /// ```
    pub fn flush(&mut self) -> Result<(), DbError> {
        self.file.flush()?;
        self.file.sync_all()?;

        Ok(())
    }  

    pub fn snapshot_write(&self, rows: &[Row], path: &Path) -> Result<(), DbError> {
        let snapshot_path = path.join("mini_db.snapshot");
        let tmp_path = path.join("mini_db.snapshot.tmp");

        let serialized = serde_json::to_string(rows)?;

        let mut tmp_file = OpenOptions::new()
                                                .create(true)
                                                .truncate(true)
                                                .write(true)
                                                .open(&tmp_path)?;

        tmp_file.write_all(serialized.as_bytes())?;
        tmp_file.flush()?;
        tmp_file.sync_all()?;

        fs::rename(tmp_path, &snapshot_path)?;

        Ok(())
                                            
    }   

    pub fn snapshot_read(&self, path: &Path) -> Result<Vec<Row>, DbError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let rows: Vec<Row> = serde_json::from_reader(reader)?;

        Ok(rows)
    }

    pub fn log_truncate(&self, path: &Path) -> Result<(), DbError> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        
        file.flush()?;
        file.sync_all()?;
        
        Ok(())
    }
}



