use mini_db::engine::Database;
use mini_db::errors::DbError;
use mini_db::model::Row;
use tempfile::tempdir;

#[test]
// Created new db and inserts row at temp path, db is dropped then initialized again and rows are fethced to test persistance
fn persistence_across_restart() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path = "temp_data.json";
    let file_path = dir.path().join(path);

    {
        let mut db = Database::new(&file_path)?;

        db.insert(1, "Alice".into(), 20)?;
        db.insert(2, "Bob".into(), 30)?;
        db.insert(3, "John".into(), 40)?;
    }

    
    let db = Database::new(&file_path)?;
    let rows = db.select_all();

    assert_eq!(rows.len(), 3);

    let mut actual = rows.clone();

    let mut expected = vec![
        Row {id: 1, name: "Alice".into(), age: 20},
        Row {id: 2, name: "Bob".into(), age: 30},
        Row {id: 3, name: "John".into(), age: 40}
    ];

    actual.sort_by_key(|row| row.id );
    expected.sort_by_key(|row| row.id);

    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn delete_persists_across_restart() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path = "temp_data.json";
    let file_path = dir.path().join(path);

    {
        let mut db: Database = Database::new(&file_path)?;

        db.insert(1, "Alice".into(), 20)?;
        db.delete_by_id(1)?;
    }

    
    let db = Database::new(&file_path)?;
    let deleted_entry = db.select_by_id(1);

    assert!(matches!(deleted_entry, Ok(None)));

    Ok(())
}

#[test]
fn reinsert_after_delete_persists() -> Result<(), DbError> {
    let dir = tempdir()?;

    let path = "temp_data.json";
    let file_path = dir.path().join(path);

    {
        let mut db = Database::new(&file_path)?;

        db.insert(1, "Alice".into(), 20)?;
        db.delete_by_id(1)?;
        db.insert(2, "Bob".into(), 30)?;
    }

    
    let db = Database::new(&file_path)?;
    
    assert!(matches!(db.select_by_id(1)?, None));
    assert_eq!(db.select_by_id(2)?, Some(Row {id: 2, name: "Bob".into(), age: 30}));

    Ok(())
}
