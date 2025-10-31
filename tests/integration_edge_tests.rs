//! Integration tests for edge cases in the mini-db database.
//!
//! These tests verify critical functionality including:
//! - Log replay and persistence across database restarts
//! - Index correctness after rebuilding from logs
//! - Index updates on insert and delete operations
//! - Performance benchmarking for common operations

use mini_db::{engine::Database, errors::DbError};
use tempfile::tempdir;

/// Tests that a large number of log entries can be successfully replayed after restart.
///
/// This test verifies that the database can handle writing and replaying 10,000 entries,
/// ensuring data persistence and log replay correctness at scale.
#[test]
fn large_log_replay_completes() -> Result<(), DbError> {
    const NUM_ENTRIES: u32 = 10_000;
    const AGE_RANGE: u32 = 50;
    const BASE_AGE: u8 = 20;
    
    let dir = tempdir()?;
    let file_path = dir.path().join("temp_data.json");
    
    // Populate database and let it go out of scope to simulate shutdown
    {
        let mut db = Database::new(&file_path)?;
        for i in 1..=NUM_ENTRIES {
            let age = BASE_AGE + (i % AGE_RANGE) as u8;
            db.insert(i, format!("User{}", i), age)?;
        }
    } // Database drops here, flushing to disk
    
    // Restart database and verify all entries were persisted
    let db = Database::new(&file_path)?;
    let rows = db.select_all();

    assert_eq!(
        rows.len(),
        NUM_ENTRIES as usize,
        "Expected {} rows after replay, found {}",
        NUM_ENTRIES,
        rows.len()
    );

    Ok(())
}

/// Tests that the index is correctly rebuilt after a database restart.
///
/// This test ensures that after restarting the database, the in-memory index
/// is accurately reconstructed from the log file, allowing all records to be
/// retrieved with their original values intact.
#[test]
fn index_rebuild_correctness() -> Result<(), DbError> {
    let dir = tempdir()?;
    let file_path = dir.path().join("index_rebuild.json");
    
    let test_data = vec![
        (1, "Alice", 30),
        (2, "Bob", 25),
        (3, "Charlie", 35),
        (4, "Diana", 28),
        (5, "Eve", 32),
    ];
    
    // Insert test data and drop database to simulate shutdown
    {
        let mut db = Database::new(&file_path)?;
        for &(id, name, age) in &test_data {
            db.insert(id, name.to_string(), age)?;
        }
    }
    
    // Restart database - this triggers index rebuild from log
    let db = Database::new(&file_path)?;
    
    // Verify every entry can be retrieved and matches expected state
    for &(id, expected_name, expected_age) in &test_data {
        let row = db.select_by_id(id)?
            .expect(&format!("Row with id {} should exist after restart", id));
        
        assert_eq!(row.id, id, "ID mismatch for entry {}", id);
        assert_eq!(row.name, expected_name, "Name mismatch for entry {}", id);
        assert_eq!(row.age, expected_age, "Age mismatch for entry {}", id);
    }
    
    Ok(())
}

/// Tests that the index is properly updated when a row is deleted.
///
/// This test verifies that:
/// 1. The index correctly removes the entry for a deleted row
/// 2. `get_index_position()` returns `None` for the deleted ID
/// 3. `select_by_id()` returns `None` for the deleted row
#[test]
fn index_update_on_delete() -> Result<(), DbError> {
    const TEST_ID: u32 = 100;
    
    let dir = tempdir()?;
    let file_path = dir.path().join("index_delete.json");
    let mut db = Database::new(&file_path)?;
    
    // Insert a test row
    db.insert(TEST_ID, "Test User".to_string(), 25)?;
    
    // Verify the index contains the inserted row
    assert!(
        db.get_index_position(TEST_ID).is_some(),
        "Index should contain id {} after insert",
        TEST_ID
    );
    
    // Delete the row
    db.delete_by_id(TEST_ID)?;
    
    // Verify the index no longer contains the deleted row
    assert!(
        db.get_index_position(TEST_ID).is_none(),
        "Index should not contain id {} after delete",
        TEST_ID
    );
    
    // Verify select_by_id also returns None
    assert!(
        db.select_by_id(TEST_ID)?.is_none(),
        "select_by_id should return None for deleted row with id {}",
        TEST_ID
    );
    
    Ok(())
}

/// Tests that the index is properly updated when a new row is inserted.
///
/// This test verifies that:
/// 1. The index correctly adds an entry for a newly inserted row
/// 2. `get_index_position()` returns `Some(position)` for the inserted ID
/// 3. The indexed position points to the correct row data
#[test]
fn index_update_on_insert() -> Result<(), DbError> {
    const TEST_ID: u32 = 42;
    const TEST_NAME: &str = "New User";
    const TEST_AGE: u8 = 30;
    
    let dir = tempdir()?;
    let file_path = dir.path().join("index_insert.json");
    let mut db = Database::new(&file_path)?;
    
    // Insert a new row
    db.insert(TEST_ID, TEST_NAME.to_string(), TEST_AGE)?;
    
    // Verify the index contains the inserted row and returns a position
    assert!(
        db.get_index_position(TEST_ID).is_some(),
        "Index should contain id {} after insert",
        TEST_ID
    );
    
    // Verify the indexed position points to the correct row data
    let row = db.select_by_id(TEST_ID)?
        .expect(&format!("select_by_id should return Some for inserted row with id {}", TEST_ID));
    
    assert_eq!(row.id, TEST_ID, "Retrieved row has incorrect ID");
    assert_eq!(row.name, TEST_NAME, "Retrieved row has incorrect name");
    assert_eq!(row.age, TEST_AGE, "Retrieved row has incorrect age");
    
    Ok(())
}

/// Performance benchmark for insert and select operations.
///
/// This test measures and reports timing metrics for:
/// - 10,000 insert operations
/// - 1,000 select operations
///
/// Results are printed to stdout for inclusion in documentation/README.
/// Note: Run with `--nocapture` flag to see output: `cargo test -- --nocapture`
#[test]
fn timing_insert_and_select() -> Result<(), DbError> {
    use std::time::Instant;
    
    const NUM_INSERTS: u32 = 10_000;
    const NUM_SELECTS: u32 = 1_000;
    const AGE_RANGE: u32 = 50;
    const BASE_AGE: u8 = 20;
    
    let dir = tempdir()?;
    let file_path = dir.path().join("timing_test.json");
    let mut db = Database::new(&file_path)?;
    
    // Benchmark insert operations
    let insert_start = Instant::now();
    for i in 1..=NUM_INSERTS {
        let age = BASE_AGE + (i % AGE_RANGE) as u8;
        db.insert(i, format!("User{}", i), age)?;
    }
    let insert_duration = insert_start.elapsed();
    
    // Benchmark select operations
    let select_start = Instant::now();
    for i in 1..=NUM_SELECTS {
        let _ = db.select_by_id(i)?;
    }
    let select_duration = select_start.elapsed();
    
    // Output performance metrics
    println!("\n=== Performance Metrics ===");
    println!("{} inserts: {:?}", NUM_INSERTS, insert_duration);
    println!("Average per insert: {:?}", insert_duration / NUM_INSERTS);
    println!("{} selects: {:?}", NUM_SELECTS, select_duration);
    println!("Average per select: {:?}", select_duration / NUM_SELECTS);
    println!("===========================\n");
    
    Ok(())
}