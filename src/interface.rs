use crate::database::Database;
use crate::network::{push_music_to_database, send_read_request, send_play_request, send_delete_peer_request};
use crate::network::notification::{Content};
use serde::{Deserialize, Serialize};
use crate::utils::{Instructions, AppListener};
use std::collections::HashMap;
use std::net::SocketAddr;
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

/// Represents a Peer in the network
#[derive(Clone)]
pub struct Peer {
    pub name: String,
    pub ip_address: SocketAddr,
    pub network_table: HashMap<String, SocketAddr>,
    pub database: Database,
    pub open_request_table: HashMap<SystemTime, Instructions>,
    pub sender: SyncSender<Notification>,
    pub redundancy_table: HashMap<SocketAddr, Vec<String>>
}

/// This function removes the Peer from the Network. Call it if you want to disconnect your
/// application gracefully while redistributing your locally saved files to the network
/// # Paramteters
/// - `peer` - The local `Peer`
pub fn delete_peer(peer: &mut Peer) {
    send_delete_peer_request(peer)
}

/// Use this function to control the playback of your music.
/// # Parameters
/// - `name` - Name of the file
/// - `peer` - The local `Peer`
/// - `state` - The desired `MusicState`
pub fn music_control(name: Option<String>, peer: &mut Peer, state: MusicState) {
    send_play_request(name, peer, state)
}

/// Use this function to play, get, order or delete a file
/// # Parameters
/// - `peer` - The local `Peer`
/// - `name` - The name of the file
/// - `istr` - The desired `Instructions`
pub fn music_request(peer: &mut Peer, name: &str, instr: Instructions) {
    send_read_request(peer, name, instr)
}

/// Use this function to upload a file to the network.
/// # Parameters
/// - `name` - The name of the file
/// - `file_path` - The relative path to the file
/// - `addr` - The local `SocketAddr`
/// - `peer` - The local `Peer`
pub fn upload_music(
    name: &str,
    file_path: &str,
    addr: SocketAddr,
    peer: &mut Peer,
) -> Result<(), io::Error> {
    push_music_to_database(name, file_path, addr, peer)
}


/// Use this function to connect to the network.
/// # Parameters
/// - `module` - A listener object that implements `AppListener` and `Sync` as a boxed value
/// - `name` - The name by which you want to be represented in the network
/// - `port` - The port you want to listen on
/// - `ip` - An optional `SocketAddr`. Pass a value if you want to join a network on that
///     `SocketAddr`. `None` if you want to start a fresh network.
///
/// # Returns
/// `Result<Arc<Mutex<Peer>>, String>`The local `Peer` in a `Mutex` if `Ok`,
/// Error message as `String` on `Err`
pub fn start(module: Box<dyn AppListener + Sync>, name: String, port: String, ip: Option<SocketAddr>) -> Result<Arc<Mutex<Peer>>, String> {
    let clone = Arc::new(Mutex::new(module));
    match network::startup(&name, &port, ip, clone) {
        Ok(p) => Ok(p),
        Err(e) => Err(e)
    }
}