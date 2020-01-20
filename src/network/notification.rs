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
    },
    SendNetworkTable {
        value: Vec<u8>,
    },
    SendNetworkUpdateTable {
        value: Vec<u8>,
    },
    RequestForTable {
        value: String,
    },
    FindFile {
        key: String,
    },
    GetFile {
        key: String,
    },
    GetFileResponse {
        key: String,
        value: Vec<u8>,
    },
    ExistFile {
        id: SystemTime,
        key: String,
    },
    ExitPeer {
        addr: SocketAddr,
    },
    DeleteFromNetwork {
        name: String,
    },
    ExistFileResponse {
        key: String,
        id: SystemTime,
    },
    StatusRequest {},
    SelfStatusRequest,
    StatusResponse {
        files: Vec<String>,
        name: String,
    },
    PlayAudioRequest {
        name: String,
    },
}
