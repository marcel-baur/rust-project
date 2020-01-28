use crate::network::response::Message;
use serde::{Deserialize, Serialize};
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
        instr: String,
        song_name: String,
    },
    GetFile {
        instr: String,
        key: String,
    },
    GetFileResponse {
        instr: String,
        key: String,
        value: Vec<u8>,
    },
    ExistFile {
        song_name: String,
        id: SystemTime,
    },
    ExitPeer {
        addr: SocketAddr,
    },
    DeleteFromNetwork {
        name: String,
    },
    ExistFileResponse {
        song_name: String,
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
    DroppedPeer {
        addr: SocketAddr,
    },
    Heartbeat,
    OrderSongRequest {
        song_name: String,
    },
}
