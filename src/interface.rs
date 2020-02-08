use crate::database::Database;
use crate::network::{get_own_ip_address, push_music_to_database, send_read_request, send_play_request, send_delete_peer_request};
use crate::network::notification::{Content};
use serde::{Deserialize, Serialize};
use crate::utils::AppListener;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::string::ToString;
use std::sync::mpsc::SyncSender;
use std::time::SystemTime;
use std::io;
use crate::network;
use std::sync::{Arc, Mutex};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum MusicState {
    PLAY,
    PAUSE,
    STOP,
    CONTINUE,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Notification {
    pub content: Content,
    pub from: SocketAddr,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Instructions {
    PLAY,
    GET,
    ORDER,
    REMOVE,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum ListenerInstr {
    NEW,
    DELETE,
    DOWNLOAD,
}

/// Represents a Peer in the network
#[derive(Clone)]
pub struct Peer {
    pub name: String,
    pub ip_address: SocketAddr,
    pub network_table: HashMap<String, SocketAddr>,
    pub database: Database,
    pub open_request_table: HashMap<SystemTime, Instructions>,
    pub sender: SyncSender<Notification>,
}

pub fn delete_peer(peer: &mut Peer) {
    send_delete_peer_request(peer)
}

pub fn music_control(name: Option<String>, peer: &mut Peer, state: MusicState) {
    send_play_request(name, peer, state)
}

pub fn music_request(peer: &mut Peer, name: &str, instr: Instructions) {
    send_read_request(peer, name, instr)
}

pub fn upload_music(
    name: &str,
    file_path: &str,
    addr: SocketAddr,
    peer: &mut Peer,
) -> Result<(), io::Error> {
    push_music_to_database(name, file_path, addr, peer)
}

pub fn start(module: Box<dyn AppListener + Sync>, name: String, port: String, ip: Option<SocketAddr>) -> Result<Arc<Mutex<Peer>>, String> {
    let clone = Arc::new(Mutex::new(module));
    match network::startup(&name, &port, ip, clone) {
        Ok(p) => Ok(p),
        Err(e) => Err(e)
    }
}