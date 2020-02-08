use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    pub from: SocketAddr,
    pub message: Message,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Message {
    DataStored { key: String },
    DataFound { key: String, value: Vec<u8> },
}
