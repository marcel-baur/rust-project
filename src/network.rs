use std::collections::HashMap;
use std::io::{stdin, Read};
use std::net::SocketAddr;
use std::net::TcpListener;
use std::thread;
use std::{io, time};
extern crate get_if_addrs;

use crate::constants;
use crate::database::*;
use crate::shell::spawn_shell;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::spawn;

/// Represents a Peer in the network
#[derive(Clone)]
pub struct Peer {
    name: String,
    ip_address: SocketAddr,
    network_table: HashMap<String, SocketAddr>,
    database: Database,
}

impl Peer {
    /// Creates a new `Peer`
    /// # Arguments:
    /// * `ip_address` - `SocketAddr` that represents the own network address
    /// * `own_name` - String that denotes the name of the Peer
    /// * `network_table` - HashMap that contains the addresses of the other Peers in the network
    pub fn create(
        ip_address: SocketAddr,
        onw_name: &str,
        network_table: HashMap<String, SocketAddr>,
    ) -> Peer {
        Peer {
            name: onw_name.to_string(),
            ip_address,
            network_table,
            database: Database::new(),
        }
    }

    pub fn get_db(&self) -> &Database {
        return &self.database;
    }
}

/// Function to create a new network
/// # Arguments:
///
/// * `own_name` - String that denotes the name of the initial Peer
///
/// # Returns:
/// A new `Peer` if successful, error string if failed
pub fn create_network(onw_name: &str) -> Result<Peer, String> {
    let ifs = match get_if_addrs::get_if_addrs() {
        Ok(v) => v,
        Err(_e) => return Err("Failed to find any network address".to_string()),
    };
    let if_options = ifs
        .into_iter()
        .find(|i| i.name == "en0".to_string() && i.addr.ip().is_ipv4());
    let this_ipv4: String = if let Some(interface) = if_options {
        interface.addr.ip().to_string()
    } else {
        "Local ip address not found".to_string()
    };
    println!("Local IP Address: {}", this_ipv4);
    let ipv4_port = format!("{}:{}", this_ipv4, "8080");
    let peer_socket_addr = match ipv4_port.parse::<SocketAddr>() {
        Ok(val) => val,
        Err(e) => return Err("Could not parse ip address to SocketAddr".to_string()),
    };
    let mut network_table = HashMap::new();
    network_table.insert(onw_name.to_string(), peer_socket_addr);
    let peer = Peer::create(peer_socket_addr, onw_name, network_table);
    let peer_clone = peer.clone();
    let handle1 = thread::Builder::new()
        .name("Listener".to_string())
        .spawn(move || {
            listen_tcp().expect("Failed to start listener");
        })
        .unwrap();
    let handle2 = thread::Builder::new()
        .name("Interaction".to_string())
        .spawn(move || {
            spawn_shell(Arc::new(Mutex::new(peer_clone))).expect("Failed to spawn shell");
        })
        .unwrap();
    handle1.join().expect_err("Handle1 failed");
    handle2.join().expect_err("Handle2 failed");

    Ok(peer)
}

pub fn join_network(onw_name: &str, ip_address: SocketAddr) {
    todo!();
}

pub fn listen_tcp() -> Result<(), String> {
    let listen_ip = "127.0.0.1:8080".to_string().parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&listen_ip).unwrap();
    for stream in listener.incoming() {
        let mut buf = vec![];
        match stream {
            Ok(mut s) => s.read_to_end(&mut buf).unwrap(),
            Err(_e) => return Err("Error".to_string()),
        };
    }
    Ok(())
}

pub fn handle_user_input(message: &str, peer: Peer) {
    println!("Handle user interaction here");
}
