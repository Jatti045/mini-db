//! Command parsing module for interactive database operations.
//!
//! This module handles parsing user input into structured commands
//! and executing those commands against the database.
//!
//! ## Supported Commands
//!
//! - `INSERT <id> <name> <age>` - Insert a new row
//! - `SELECT` - Retrieve all rows
//! - `SELECT WHERE ID=<id>` - Retrieve a specific row by ID
//! - `DELETE WHERE ID=<id>` - Delete a row by ID
//! - `EXEC BATCH <path>` - Execute commands from a file
//! - `RESET` - Clear all data
//! - `HELP` - Display help information
//! - `EXIT` - Shutdown and exit

use std::path::{PathBuf};
use crate::engine::Database;
use crate::model::Row;
use crate::errors::DbError;

/// Represents a parsed database command.
///
/// Each variant corresponds to a user-facing command and contains
/// the necessary parameters for execution.
#[derive(PartialEq, Debug)]
pub enum Command {
    /// Insert a new row with the specified values
    Insert {
        id: u32,
        name: String,
        age: u8,
    },
    /// Execute a batch of commands from a file
    ExecBatch {
        path: PathBuf,
    },
    /// Select a specific row by its ID
    SelectById {
        id: u32,
    },
    /// Delete a specific row by its ID
    DeleteById {
        id: u32,
    },
    /// Select and display all rows
    Select,
    /// Exit the program
    Exit,
    /// Display help information
    Help,
    /// Reset (clear) the entire database
    Reset,
}

/// Parses a string input into a structured Command.
///
/// Commands are case-insensitive and whitespace-separated.
///
/// # Arguments
///
/// * `input` - The raw user input string
///
/// # Returns
///
/// Returns `Ok(Command)` if parsing succeeds, or a `DbError` if:
/// - The command syntax is invalid
/// - Required parameters are missing or malformed
/// - Numeric values cannot be parsed
///
/// # Examples
///
/// ```
/// use mini_db::parser::parse_command;
///
/// let cmd = parse_command("INSERT 1 Alice 30").unwrap();
/// let cmd = parse_command("SELECT WHERE ID=1").unwrap();
/// let cmd = parse_command("DELETE WHERE ID=1").unwrap();
/// ```
pub fn parse_command(input: &str) -> Result<Command, DbError> {
    let line = input.trim().to_lowercase();

    if line.is_empty() {
        return Err(DbError::InvalidCommandError);
    }

    // Tokenize input by whitespace
    let tokens: Vec<&str> = line.split_whitespace().collect();
    let cmd = tokens[0];

    match cmd {
        "exec" => {
            if tokens.len() == 3 && tokens[1] == "batch" {
                let path = PathBuf::from(tokens[2]);
                Ok(Command::ExecBatch { path })
            } else {
                Err(DbError::InvalidCommandError)
            }
        },
        "insert" => {
            if tokens.len() == 4 {
                // Parse: INSERT <id> <name> <age>
                let id: u32 = tokens[1].parse().map_err(|_| {
                    DbError::ParseError("ID must be a valid unsigned integer".to_string())
                })?;

                let name = tokens[2].to_string();

                let age: u8 = tokens[3].parse().map_err(|_| {
                    DbError::ParseError("Age must be a valid integer (0-255)".to_string())
                })?;

                Ok(Command::Insert { id, name, age }) 
            } else {
                Err(DbError::InvalidCommandError)
            }
        },
        "select" => {
            if tokens.len() == 1 && tokens[0] == "select" {
                return Ok(Command::Select);
            } else if tokens.len() == 3 && tokens[1] == "where" && tokens[2].starts_with("id=") {
                let id: u32 = match tokens[2].split("=").nth(1) {
                Some(id) => id.parse().map_err(|_| {
                    DbError::ParseError("Id not found".to_string())
                })?,
                    None => return Err(DbError::ParseError("Id not found".into()))
                };
                return Ok(Command::SelectById { id });
            } else {
                Err(DbError::InvalidCommandError)
            }
        },
        "delete" => {
             if tokens.len() == 3 && tokens[1] == "where" && tokens[2].starts_with("id=") {
                let id: u32 = match tokens[2].split("=").nth(1) {
                Some(id) => id.parse().map_err(|_| {
                    DbError::ParseError("Id not found".to_string())
                })?,
                    None => return Err(DbError::ParseError("Id not found".into()))
                };
                return Ok(Command::DeleteById { id });
            } else {
                Err(DbError::InvalidCommandError)
            }
        }
        "exit" => Ok(Command::Exit),
        "help" => Ok(Command::Help),
        "reset" => Ok(Command::Reset),
        _ => Err(DbError::InvalidCommandError)
    }

}

/// Parses and executes a command against the database.
///
/// This is the main entry point for command execution. It:
/// 1. Parses the input string into a Command
/// 2. Executes the command against the database
/// 3. Prints results or error messages
///
/// # Arguments
///
/// * `input` - The raw command string from the user
/// * `db` - A mutable reference to the database
///
/// # Returns
///
/// Returns `true` if the program should continue running,
/// `false` if the user issued an EXIT command.
///
/// # Examples
///
/// ```no_run
/// use mini_db::engine::Database;
/// use mini_db::parser::handle_command;
///
/// let mut db = Database::new("data.json")?;
/// let should_continue = handle_command("INSERT 1 Alice 30", &mut db);
/// # Ok::<(), mini_db::errors::DbError>(())
/// ```
pub fn handle_command(input: &str, db: &mut Database) -> bool {
    match parse_command(input) {
        Ok(Command::Insert {id, name, age}) => {
            match db.insert(id, name, age) {
                Ok(()) => println!("Inserted row with id {id}."),
                Err(e) => eprintln!("Error inserting into db: {}", e),
            }
            true
        },

        Ok(Command::ExecBatch { path }) => {
            match db.exec_batch(path) {
                Ok(()) => println!("Batch commands executed successfully."),
                Err(e) => println!("Error executing batch commands: {}", e)
            }
            true
        },

        Ok(Command::SelectById { id }) => {
            match db.select_by_id(id) {
                Ok(Some(row)) => println!("{:?}", row),
                Ok(None) => println!("Row with id {} not found.", id),
                Err(e) => eprintln!("Error fetching row by id: {}", e)
            }
            true
        },

        Ok(Command::DeleteById { id }) => {
            match db.delete_by_id(id) {
                Ok(true) => println!("Row with id {} deleted.", id),
                Ok(false) => println!("No row found with id {}.", id),
                Err(e)    => eprintln!("Error deleting row: {}", e),
            }   
            true
        },

        Ok(Command::Select) => {
            let rows: &Vec<Row> = db.select_all();

            if rows.len() == 0 {
                println!("(no rows)");
                return true;
            }

            for row in rows.iter() {
                println!("{:?}", row)
            }
            true
        }, 

        Ok(Command::Exit) => {
            if let Err(e) = db.shutdown() {
                println!("Warning: could not flush data: {}", e);
            }
            println!("Exiting mini_db... Goodbye!");
            false
        },

        Ok(Command::Help) => {
            println!("\nAvailable commands:\nEXEC BATCH <FILEPATH.TXT>\nINSERT <ID> <NAME> <AGE>\nSELECT\nSELECT WHERE ID=<ID>\nDELETE WHERE ID=<ID>\nRESET\nEXIT\n");
            true
        },

        Ok(Command::Reset) => {
            match db.reset_db() {
                Ok(_) => println!("All data cleared."),
                Err(_) => println!("Database could not be reset."),
            }
            true
        }

        Err(_) => {
            println!("Enter a valid command");
            true
        },
    }


}