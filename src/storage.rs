use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};
use chrono::Utc;

use crate::model::Row;
use crate::errors::DbError;

#[derive(Serialize, Deserialize, Debug)]
pub enum LogEntry {
    Insert {
        row: Row,
        timestamp: i64
    },
    Delete {
        id: u32,
    }
}

/// Defines our storage
pub struct Storage {
    // Storage path
    pub path: PathBuf,
    pub file: File
}

impl Storage {
    /// Initializes storage at path
    pub fn new(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let path = path.as_ref().to_path_buf();

        // Appends to EOF and created file if DNE
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)?;

        Ok(Storage {
            path,
            file
        })
    }

    /// Adds new entry to our storage
    pub fn append_entry(&mut self, row: &Row) -> Result<(), DbError> {
        let log_entry = LogEntry::Insert {
            row: row.clone(),
            timestamp: Utc::now().timestamp(),
        };

        // Serialize row into json string format
        let json = serde_json::to_string(&log_entry)?;

        // Write json to file with new line appended at end
        writeln!(self.file, "{}", json)?;    

        Ok(())
    } 

    pub fn append_delete(&mut self, id:u32) -> Result<(), DbError> {
        let log_entry = LogEntry::Delete { id };

        // Serialize row into json string format
        let json = serde_json::to_string(&log_entry)?;

        // Write json to file with new line appended at end
        writeln!(self.file, "{}", json)?;

        Ok(())
    }


    /// Loads all data from storage 
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
                Ok(LogEntry::Insert {row, timestamp}) => rows.push(row),
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

    /// Ensures all pending writes are flushed and synched to disk
    pub fn flush(&mut self) -> Result<(), DbError> {
        self.file.flush()?;
        self.file.sync_all()?;

        Ok(())
    }   
}



