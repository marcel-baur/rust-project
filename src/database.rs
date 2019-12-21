use std::collections::HashMap;
use std::fmt::Binary;
use std::fs::File;

pub struct Database {
    data: HashMap<String, File>
}
impl Database {
    pub fn new() -> Database {
        let fresh_db: HashMap<String, File> = HashMap::new();
        return Database {
            data: fresh_db
        };
    }
}
