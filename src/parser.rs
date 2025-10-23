use crate::errors::DbError;

#[derive(PartialEq, Debug)]
/// Defines available commands
pub enum Command {
    Insert {
        id: u32,
        name: String,
        age: u8,
    },
    Select,
    Exit,
    Help,
}

/// Parses user input and determines which command is to be executed
pub fn parse_command(input: &str) -> Result<Command, DbError> {
    let line = input.trim().to_lowercase();

    if line.is_empty() {
        return Err(DbError::InvalidCommand);
    }

    // Split into tokens
    let tokens:Vec<&str> = line.split_whitespace().collect();
    let cmd = tokens[0];

    match cmd {
        "insert" => {
            if tokens.len() != 4 {
                return Err(DbError::InvalidCommand);
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
        }
        "select" => Ok(Command::Select),
        "exit" => Ok(Command::Exit),
        "help" => Ok(Command::Help),
        _ => Err(DbError::InvalidCommand)
    }

}