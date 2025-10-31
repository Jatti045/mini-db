use mini_db::{errors::DbError, parser };

#[test]
fn parse_insert_command_valid() -> Result<(), DbError> {
    let input = "insert 1 alice 20";
    let cmd = parser::parse_command(&input)?;

    assert_eq!(cmd, parser::Command::Insert { id: 1, name: "alice".into(), age: 20 });

    Ok(())
}

#[test]
fn parse_select_command_valid() -> Result<(), DbError> {
    let input = "select";
    let cmd = parser::parse_command(&input)?;

    assert_eq!(cmd, parser::Command::Select);

    Ok(())
}

#[test]
fn parse_select_where_command_valid() -> Result<(), DbError> {
    let input = "select where id=5";
    let cmd = parser::parse_command(&input)?;

    assert_eq!(cmd, parser::Command::SelectById { id:5 });

    Ok(())
}

#[test]
fn parse_delete_command_valid() -> Result<(), DbError> {
    let input = "delete where id=5";
    let cmd = parser::parse_command(&input)?;

    assert_eq!(cmd, parser::Command::DeleteById { id:5 });

    Ok(())
}

#[test]
fn parse_invalid_missing_id() -> Result<(), DbError> {
    let input = "select where";
    let cmd = parser::parse_command(&input);

    assert!(matches!(cmd, Err(DbError::InvalidCommandError)));

    Ok(())
}

#[test]
fn parse_invalid_non_number_id() -> Result<(), DbError> {
    let input = "delete where id=abc";
    let cmd = parser::parse_command(&input);

    assert!(matches!(cmd, Err(DbError::ParseError(_))));

    Ok(())
}