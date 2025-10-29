use crate::errors::DbError;

#[derive(PartialEq, Debug)]
/// Defines available commands
pub enum Command {
    Insert {
        id: u32,
        name: String,
        age: u8,
    },
    SelectById {
        id: u32,
    },
    DeleteById {
        id: u32,
    },
    Select,
    Exit,
    Help,
    Reset,
}

/// Parses user input and determines which command is to be executed
pub fn parse_command(input: &str) -> Result<Command, DbError> {
    let line = input.trim().to_lowercase();

    if line.is_empty() {
        return Err(DbError::InvalidCommandError);
    }

    // Split into tokens
    let tokens:Vec<&str> = line.split_whitespace().collect();
    let cmd = tokens[0];

    match cmd {
        "insert" => {
            if tokens.len() != 4 {
                return Err(DbError::InvalidCommandError);
            }

            // Parse id, name, age
            let id: u32 = tokens[1].parse().map_err(|_| {
                DbError::ParseError("Id must be an integer".to_string())
            })?;

            let name = tokens[2].to_string();

            let age: u8 = tokens[3].parse().map_err(|_| {
                DbError::ParseError("Age must be an integer".to_string())
            })?;

            Ok(Command::Insert{id, name, age}) 
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