use std::collections::HashMap;
use std::io::{stdin, Read, Write};
use std::net::TcpListener;
use std::net::{SocketAddr, TcpStream};
use std::thread;
use std::{io, time};
extern crate get_if_addrs;
use crate::constants;
use crate::database::*;
use crate::shell::spawn_shell;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

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

    pub fn store(self, data: (String, Vec<u8>)) {
        let k = data.0;
        let v = data.1;
        self.database.add_file(&k, v);
    }

    pub fn get_ip(&self) -> &SocketAddr {
        return &self.ip_address;
    }

    pub fn get_db(&self) -> &Database {
        return &self.database;
    }

    pub fn process_store_request(&mut self, data: (String, Vec<u8>)) {
        self.database.data.insert(data.0, data.1);
    }
}

pub fn get_own_ip_address() -> Result<SocketAddr, String> {
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
    let ipv4_port = format!("{}:{}", this_ipv4, "1289");
    let peer_socket_addr = match ipv4_port.parse::<SocketAddr>() {
        Ok(val) => val,
        Err(e) => return Err("Could not parse ip address to SocketAddr".to_string()),
    };
    println!("Local IP Address: {}", this_ipv4);
    Ok(peer_socket_addr)
}

/// Function to create a new network
/// # Arguments:
///
/// * `own_name` - String that denotes the name of the initial Peer
///
/// # Returns:
/// A new `Peer` if successful, error string if failed
pub fn create_peer(onw_name: &str) -> Result<Peer, String> {
    let peer_socket_addr = match get_own_ip_address() {
        Ok(val) => val,
        Err(error_message) => return Err(error_message)
    };
    let mut network_table = HashMap::new();
    network_table.insert(onw_name.to_string(), peer_socket_addr);
    let peer = Peer::create(peer_socket_addr, onw_name, network_table);
    Ok(peer)
}

pub fn startup(name: String) -> JoinHandle<()> {
    let concurrent_thread = thread::Builder::new().name("ConThread".to_string());
    concurrent_thread
        .spawn(move || {
            let peer = create_peer(name.as_ref()).unwrap();
            let peer_arc = Arc::new(Mutex::new(peer));
            let peer_arc_clone_listen = peer_arc.clone();
            let listener = thread::Builder::new()
                .name("TCPListener".to_string())
                .spawn(move || {
                    listen_tcp(peer_arc_clone_listen);
                })
                .unwrap();
            let peer_arc_clone_interact = peer_arc.clone();
            let interact = thread::Builder::new()
                .name("Interact".to_string())
                .spawn(move || {
                    spawn_shell(peer_arc_clone_interact);
                })
                .unwrap();
            listener.join().expect_err("Could not join Listener");
            interact.join().expect_err("Could not join Interact");
        })
        .unwrap()
}

pub fn join_network(onw_name: &str, ip_address: SocketAddr) -> Result<(), String> {
    //get own ip address
    let peer_socket_addr = match get_own_ip_address() {
        Ok(val) => val,
        Err(error_message) => return Err(error_message)
    };
    //get existing hashmap table
    send_table_request(ip_address, peer_socket_addr);
    Ok(())
}

pub fn listen_tcp(arc: Arc<Mutex<Peer>>) -> Result<(), String> {
    let listen_ip = "0.0.0.0:34254".to_string().parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&listen_ip).unwrap();
    let clone = arc.clone();
    println!("Listening on {}", listen_ip);
    for stream in listener.incoming() {
        println!("Received");
        let mut buf = String::new();
        //        dbg!(&stream);
        match stream {
            Ok(mut s) => {
                s.read_to_string(&mut buf).unwrap();
                //                println!("Buffer: {}", &buf);
                let deserialized: SendRequest = match serde_json::from_str(&buf) {
                    Ok(val) => val,
                    Err(e) => {
                        println!("{}", e.to_string());
                        SendRequest {
                            key: "key".to_string(),
                            from: "from".to_string(),
                            value: Vec::new(),
                            action: "write".to_string(),
                        }
                    }
                };
                let mut peer = clone.lock().unwrap();
                dbg!(&deserialized);
                handle_incoming_requests(&deserialized);
                //peer.process_store_request((deserialized.key, deserialized.value));
                drop(peer);
                println!("Done Writing");
                // TODO: Response, handle duplicate key, redundancy
            }
            Err(_e) => {
                println!("could not read stream");
                return Err("Error".to_string());
            }
        };
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendRequest {
    value: Vec<u8>,
    from: String,
    key: String,
    action: String,
}

impl fmt::Display for SendRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\"key\": \"{:?}\", \"value\": \"{:?}\", \"action\": {:?}",
            self.key, self.value, self.action
        )
    }
}

//request to get hashmap table
pub fn send_table_request(target: SocketAddr, from: SocketAddr) {
    let mut stream = TcpStream::connect(target).unwrap();
    let buf = SendRequest {
        value: vec![],
        from: from.to_string(),
        key: "".to_string(),
        action: "get_network_table".to_string(),
    };
    let serialized = match serde_json::to_writer(&stream, &buf) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SimpleRequest {:?}", &buf);
        }
    };
}

pub fn handle_incoming_requests(request: &SendRequest) {
    match request.action.as_ref() {
        "get_network_table" => {
            println!("network table request");
        }
        _ => {println!("no valid request");}
    }
}

pub fn send_write_request(target: SocketAddr, data: (String, Vec<u8>)) {
    let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();
    let mut vec: Vec<u8> = Vec::new();
    vec.push(1);
    vec.push(0);
    let buf = SendRequest {
        value: data.1,
        key: data.0,
        from: target.to_string(),
        action: "write".to_string(),
    };
    let serialized = match serde_json::to_writer(&stream, &buf) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &buf);
        }
    };
}
