use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SendRequest {
    pub value: Vec<u8>,
    pub from: String,
    pub key: String,
    pub action: String,
}

impl fmt::Display for SendRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\"key\": \"{:?}\", \"value\": \"{:?}\", \"action\": {:?}",
            self.key, self.value, self.action
        )
    }
}
