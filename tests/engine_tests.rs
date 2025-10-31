use mini_db::engine::Database;
use mini_db::errors::DbError;
use mini_db::model::Row;
use tempfile::tempdir;


#[test]
fn insert_and_select_all() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path = "temp_data.json";
    let file_path = dir.path().join(path);

    let mut db = Database::new(&file_path)?;

    let first_entry = Row {id: 1, name: "name1".into(), age: 20};
    let second_entry = Row {id: 2, name: "name2".into(), age: 30};
    let thrid_entry =Row {id: 3, name: "name3".into(), age: 40};

    let entries = vec![first_entry, second_entry, thrid_entry];

    db.insert(1, "name1".into(), 20)?;
    db.insert(2, "name2".into(), 30)?;
    db.insert(3, "name3".into(), 40)?;

    let rows = db.select_all();

    assert_eq!(rows, &entries);

    Ok(())
}

#[test]
fn insert_and_select_by_id() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path = "temp_data.json";
    let file_path = dir.path().join(path);

    let mut db = Database::new(&file_path)?;

    db.insert(1, "name1".into(), 20)?;
    db.insert(2, "name2".into(), 30)?;
    db.insert(3, "name3".into(), 40)?;

    let first_entry = Row {id: 1, name: "name1".into(), age: 20};

    let selected_row = db.select_by_id(1)?;

    assert_eq!(selected_row, Some(first_entry));

    Ok(())
}

#[test]
fn delete_existing_row() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path = "temp_data.json";
    let file_path = dir.path().join(path);

    let mut db = Database::new(&file_path)?;

    db.insert(1, "name1".into(), 20)?;

    let deleted_row = db.delete_by_id(1)?;
    let select_deleted_row = db.select_by_id(1)?;

    assert!(deleted_row == true);
    assert_eq!(select_deleted_row, None);

    Ok(())
}

#[test]
fn delete_nonexistent_row() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path = "temp_data.json";
    let file_path = dir.path().join(path);

    let mut db = Database::new(&file_path)?;

    let deleted_row = db.delete_by_id(1)?;

    assert!(!deleted_row);

    Ok(())
}

#[test]
fn delete_then_reinsert() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path = "temp_data.json";
    let file_path = dir.path().join(path);

    let mut db = Database::new(&file_path)?;

    db.insert(1, "name1".into(), 20)?;
    db.delete_by_id(1)?;
    db.insert(1, "name1".into(), 20)?;

    let rows = db.select_all();

    assert_eq!(rows.len(), 1);

    Ok(())
}

#[test]
fn prevent_duplicate_id_insert() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path = "temp_data.json";
    let file_path = dir.path().join(path);

    let mut db = Database::new(&file_path)?;

    db.insert(1, "name1".into(), 20)?;
    let err = db.insert(1, "name2".into(), 30);

    assert!(matches!(err, Err(DbError::DuplicateIdError(_))));

    Ok(())
}