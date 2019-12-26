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
use local_ipaddress;
use std::str::FromStr;

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

#[cfg(target_os = "macos")]
fn get_own_ip_address(port: &str) -> Result<SocketAddr, String> {
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
    let ipv4_port = format!("{}:{}", this_ipv4, port);
    let peer_socket_addr = match ipv4_port.parse::<SocketAddr>() {
        Ok(val) => val,
        Err(e) => return Err("Could not parse ip address to SocketAddr".to_string()),
    };
    Ok(peer_socket_addr)
}

// This function only gets compiled if the target OS is linux
#[cfg(not(target_os = "macos"))]
fn get_own_ip_address(port: &str) -> Result<SocketAddr, String> {
    let this_ipv4 = match local_ipaddress::get() {
        Some(val) => val,
        None => return Err("Failed to find any network address".to_string())
    };
    println!("Local IP Address: {}", this_ipv4);
    let ipv4_port = format!("{}:{}", this_ipv4, port);
    let peer_socket_addr = match ipv4_port.parse::<SocketAddr>() {
        Ok(val) => val,
        Err(e) => return Err("Could not parse ip address to SocketAddr".to_string()),
    };
    Ok(peer_socket_addr)
}


/// Function to create a new network
/// # Arguments:
///
/// * `own_name` - String that denotes the name of the initial Peer
///
/// # Returns:
/// A new `Peer` if successful, error string if failed
pub fn create_peer(onw_name: &str, port: &str) -> Result<Peer, String> {
    let peer_socket_addr = match get_own_ip_address(port) {
        Ok(val) => val,
        Err(error_message) => return Err(error_message)
    };
    let mut network_table = HashMap::new();
    network_table.insert(onw_name.to_string(), peer_socket_addr);
    let peer = Peer::create(peer_socket_addr, onw_name, network_table);
    Ok(peer)
}

pub fn startup(name: String, port: String) -> JoinHandle<()> {
    let concurrent_thread = thread::Builder::new().name("ConThread".to_string());
    concurrent_thread
        .spawn(move || {
            let peer = create_peer(name.as_ref(), port.as_ref()).unwrap();
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

pub fn join_network(own_name: &str, port: &str, ip_address: SocketAddr) -> Result<(), String> {
    //get own ip address
    let own_addr = match get_own_ip_address(port) {
        Ok(val) => val,
        Err(error_message) => return Err(error_message)
    };

    //open new tcp listener for incoming network tables
    let listener = thread::Builder::new()
        .name("table_request".to_string())
        .spawn(move || {
            listen_tcp_for_handshake(&own_addr);
        })
        .unwrap();

    //send request existing network table
    send_table_request(ip_address, own_addr, own_name);

    listener.join();
    
    Ok(())
}

pub fn listen_tcp_for_handshake(ip_address: &SocketAddr) -> Result<(), String> {
    let listen_ip = ip_address.to_string().parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&listen_ip).unwrap();
    println!("Listening on {}", listen_ip);
    for stream in listener.incoming() {
        let mut buf = String::new();
        match stream {
            Ok(mut s) => {
                s.read_to_string(&mut buf).unwrap();
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
                //dbg!(&deserialized);
                let string = String::from_utf8(deserialized.value).unwrap();
                println!("got the network table");
                let networkTable = json_string_to_network_table(string);
                dbg!(&networkTable);
                // here exit thread to join and create peer
                break
            }
            Err(_e) => {
                println!("could not read stream");
                return Err("Error".to_string());
            }
        };
    }
    Ok(())
}

pub fn listen_tcp(arc: Arc<Mutex<Peer>>) -> Result<(), String> {
    let clone = arc.clone();
    let listen_ip = clone.lock().unwrap().ip_address;
    let listener = TcpListener::bind(&listen_ip).unwrap();
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
                handle_incoming_requests(deserialized, &peer);
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
pub fn send_table_request(target: SocketAddr, from: SocketAddr, name: &str) {
    println!("sending table request");
    let mut stream = match TcpStream::connect(target) {
        Ok(stream) => stream,
        Err(e) => {
            dbg!(e);
            return;
        }
    };
    let buf = SendRequest {
        value: name.as_bytes().to_vec(),
        from: from.to_string(),
        key: "name".to_string(),
        action: "get_network_table".to_string(),
    };
    let serialized = match serde_json::to_writer(&stream, &buf) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SimpleRequest {:?}", &buf);
        }
    };
}

pub fn handle_incoming_requests(request: SendRequest, peer: &Peer) {
    let copy = request.value;
    match request.action.as_ref() {
        "get_network_table" => {
            let name = match String::from_utf8(copy) {
                Ok(val) => val,
                Err(utf) => return
            };
            //@TODO check name in network table with hashmap contains key function
            println!("network table request with name: {}", name);
            send_network_table(request.from, &peer);
        }
        _ => {println!("no valid request");}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkInfo {
    name: String,
    address: String
}

pub fn json_string_to_network_table(json_string: String) -> HashMap<String, SocketAddr> {
    let info_array: Vec<NetworkInfo> = match serde_json::from_str(json_string.as_str()) {
        Ok(val) => val,
        Err(e) => {
            println!("no parcing hashmap");
            return HashMap::new();
        }
    };
    dbg!(&info_array);
    let mut hashmap = HashMap::new();
    for info in info_array {
        let key = info.name;
        let addr = SocketAddr::from_str(info.address.as_str()).unwrap();
        hashmap.insert(key, addr);
    }
    hashmap
}

pub fn network_table_to_json(network_table: &HashMap<String, SocketAddr>) -> String {
    let mut array = vec![];
    for (key, address) in network_table {
        array.push(NetworkInfo { name: key.clone(), address: address.clone().to_string() } );
    }
    serde_json::to_string( &array).unwrap()
}

pub fn send_network_table(target: String, peer: &Peer) {
    let mut stream = TcpStream::connect(target).unwrap();
    let buf = SendRequest {
        value: network_table_to_json(&peer.network_table).into_bytes(),
        key: "network_table".to_string(),
        from: peer.ip_address.to_string(),
        action: "ack_network_table".to_string(),
    };
    match serde_json::to_writer(&stream, &buf) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &buf);
        }
    };
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
