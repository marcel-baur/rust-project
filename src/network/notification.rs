use crate::network::response::Message;
use crate::network::send_request::SendRequest;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub content: Content,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Content {
    SendRequest {
        key: String,
        value: Vec<u8>,
        action: String,
        from: String,
    },
    Response {
        from: SocketAddr,
        message: Message,
    },
}
