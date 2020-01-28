use crate::network::response::Message;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, TcpStream};
use std::time::SystemTime;
use crate::utils::Instructions;

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
        instr: Instructions,
        song_name: String,
    },
    GetFile {
        instr: Instructions,
        key: String,
    },
    GetFileResponse {
        instr: Instructions,
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
    DeleteFileRequest {
        song_name: String,
    }
}

pub fn tcp_request_with_notification(target: SocketAddr, notification: Notification) {
    let stream = match TcpStream::connect(target) {
        Ok(s) => s,
        Err(_e) => {
            eprintln!("Failed to connect to {:?}", target);
            return;
        }
    };

    let not = notification;

    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };

}
