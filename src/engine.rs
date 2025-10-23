use crate::model::Row;
use crate::errors::DbError;

/// Defines our database
pub struct Database {
    rows: Vec<Row>
}

impl Database {
    /// Database constructor
    pub fn new() -> Self {
        Self {
            rows: Vec::new()
        }
    }

    /// Insert new row into database by id, name, and age
    pub fn insert(&mut self, id: u32, name: String, age: u8) -> Result<(), DbError> {

        // Iterates over database rows and verifies whether id of row to be inserted exists in db
        if self.rows.iter().any(|r| r.id == id) {
            return Err(DbError::DuplicateId(id));
        }

        let newly_created_row = Row::new(id, name, age);
        self.rows.push(newly_created_row);

        Ok(())
    }

    /// Returns all rows from the database
    pub fn select_all(&self) -> &Vec<Row> {
        &self.rows
    }
}