
#[derive(Debug, Clone, PartialEq)]
/// Defines each database row
pub struct Row {
    pub id: u32,
    pub name: String,
    pub age: u8,
}

impl Row {
    /// Row constructor
    pub fn new(id: u32, name: String, age: u8) -> Self {
        Self {
            id,
            name,
            age,
        }
    }
}