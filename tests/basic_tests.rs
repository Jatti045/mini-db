use mini_db::engine::Database;
use mini_db::model::Row;
use mini_db::errors::DbError;
use mini_db::parser;
use tempfile::tempdir;

#[test]
// A newly created database has 0 rows
fn new_db_starts_empty() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path: &'static str = "temp_data.json";
    let file_path = dir.path().join(path);

    let db = Database::new(file_path)?;
    let rows = db.select_all();
    assert_eq!(rows.len(), 0, "Database should start empty");

    Ok(())
}

#[test]
// Inserting row into newly created database has row length of 1
fn insert_adds_new_row() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path: &'static str = "temp_data.json";
    let file_path = dir.path().join(path);

    let mut db = Database::new(file_path)?;

    db.insert(1, "Alice".into(), 20)?;

    let rows = db.select_all();
    assert_eq!(rows.len(), 1, "expected one row after insert");

    Ok(())
}

#[test]
// Returns all rows from database
fn select_returns_all_inserted_rows() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path: &'static str = "temp_data.json";
    let file_path = dir.path().join(path);

    let mut db = Database::new(file_path)?;

    // Create row0 and row1 which will be used to compare with rows inserted into database from select_all (Rows with same content as row0 and row1 will be inserted into database)
    let row0 = Row {
        id: 1,
        name: "name".into(),
        age: 20,
    };

    let row1 = Row {
        id: 2,
        name: "name".into(),
        age: 30,
    };

    // Insert two rows into db
    db.insert(1, "name".to_string(), 20)?;
    db.insert(2, "name".to_string(), 30)?;

    let rows = db.select_all();

    assert_eq!(rows, &vec![row0, row1]);

    Ok(())
}

#[test]
// Attempts to add a new row into database with an existing id
fn insert_rejects_duplicate_ids() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path: &'static str = "temp_data.json";
    let file_path = dir.path().join(path);
    
    let mut db = Database::new(file_path)?;

    db.insert(1, "name".to_string(), 20)?;

    let err  = db.insert(1, "name".to_string(), 30);

    assert!(matches!(err, Err(DbError::DuplicateIdError(1))));

    Ok(())
}

#[test]
// Tests parsing logic and ensures correct command for user input is returned
fn parse_valid_insert_returns_command() -> Result<(), DbError> {
    let input = "insert 1 alice 30";

    let command = parser::parse_command(input)?;
    assert_eq!(command, parser::Command::Insert { id: 1, name: "alice".into(), age: 30 });

    Ok(())
}

