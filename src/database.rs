use std::collections::HashMap;
use std::fmt::Binary;
use std::fs::File;

#[derive(Clone)]
pub struct Database {
    pub data: HashMap<String, Vec<u8>>,
}
impl Database {
    pub fn new() -> Database {
        let fresh_db: HashMap<String, Vec<u8>> = HashMap::new();
        return Database { data: fresh_db };
    }

    pub fn clone() {}

    pub fn get_data(&self) -> &HashMap<String, Vec<u8>> {
        return &self.data;
    }
}
