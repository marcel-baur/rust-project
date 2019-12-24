use crate::network::Peer;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::Binary;
use std::fs::File;

#[derive(Clone)]
pub struct Database {
    pub data: HashMap<String, Vec<u8>>,
}
impl Database {
    pub fn new() -> Database {
        Database {
            data: HashMap::new(),
        }
    }

    pub fn add_file(mut self, key: &str, content: Vec<u8>) {
        self.data.insert(key.to_string(), content);
    }

    pub fn get_file(&self, key: &str) -> Option<&Vec<u8>> {
        self.data.get(key)
    }

    pub fn get_data(&self) -> &HashMap<String, Vec<u8>> {
        return &self.data;
    }
}
