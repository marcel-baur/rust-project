use crate::interface::MusicState;
use crate::network::response::Message;
use crate::interface::*;
use crate::utils::Instructions;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, TcpStream};
use std::process;
use std::time::{SystemTime, Duration};

#[derive(Clone, Serialize, Deserialize, Debug)]
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
        name: Option<String>,
        state: MusicState,
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
    },
}

pub fn tcp_request_with_notification(target: SocketAddr, notification: Notification) {
    let stream = match TcpStream::connect_timeout(&target, Duration::new(30, 1)) {
        Ok(s) => s,
        Err(_e) => {
            println!("{:?}", _e);
            handle_error(notification.content, target);
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

fn handle_error(content: Content, target: SocketAddr) {
    match content {
        Content::RequestForTable { .. } => {
            println!("There is no existing network containing this IP {:?}\nPlease check the IP-Address you want to join", target);
            process::exit(0);
        }
        _ => {
            eprintln!("Failed to connect to {:?}", target);
        }
    }
}
