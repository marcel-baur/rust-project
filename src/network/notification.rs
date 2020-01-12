use crate::network::response::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub content: Content,
    pub from: SocketAddr,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Content {
    PushToDB {
        key: String,
        value: Vec<u8>,
        from: String,
    },
    RedundantPushToDB {
        key: String,
        value: Vec<u8>,
        from: String,
    },
    Response {
        from: SocketAddr,
        message: Message,
    },
    ChangePeerName {
        value: String,
        from: SocketAddr,
    },
    SendNetworkTable {
        value: Vec<u8>,
        from: SocketAddr,
    },
    SendNetworkUpdateTable {
        value: Vec<u8>,
        from: SocketAddr,
    },
    RequestForTable {
        value: String,
        from: SocketAddr,
    },
    FindFile {
        key: String,
    },
    ExistFile {
        id: SystemTime,
        key: String,
        from: SocketAddr,
    },
    ExitPeer {
        addr: SocketAddr,
    },
    DeleteFromNetwork {
        name: String,
        from: SocketAddr,
    },
    ExistFileResponse {
        key: String,
        from: SocketAddr,
        exist: bool,
    },
}
