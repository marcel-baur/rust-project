use std::collections::HashMap;

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

    pub fn get_data(&self) -> &HashMap<String, Vec<u8>> {
        &self.data
    }
}
