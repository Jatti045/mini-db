use mini_db::engine::Database;
use mini_db::errors::DbError;
use mini_db::model::Row;
use tempfile::tempdir;

#[test]
// Created new db and inserts row at temp path, db is dropped then initialized again and rows are fethced to test persistance
fn test_persistance_across_sessions() -> Result<(), DbError> {
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

