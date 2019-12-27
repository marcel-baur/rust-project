use std::collections::HashMap;
use std::io::{stdin, Read, Write};
use std::net::TcpListener;
use std::net::{SocketAddr, TcpStream};
use std::thread;
use std::{io, time};
use std::borrow::Borrow;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use local_ipaddress;
use std::str::FromStr;

pub mod peer;
mod handshake;
mod send_request;

extern crate get_if_addrs;

use crate::constants;
use crate::database::*;
use crate::shell::spawn_shell;
use crate::network::handshake::{json_string_to_network_table, network_table_to_json, send_network_table, send_table_request};
use crate::network::send_request::SendRequest;
use crate::network::peer::{create_peer, Peer};

#[cfg(target_os = "macos")]
pub fn get_own_ip_address(port: &str) -> Result<SocketAddr, String> {
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
pub fn get_own_ip_address(port: &str) -> Result<SocketAddr, String> {
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
    let peer = create_peer(own_name, port.as_ref()).unwrap();
    let own_addr = peer.ip_address.clone();
    let peer_arc = Arc::new(Mutex::new(peer));
    let peer_arc_clone_listen = peer_arc.clone();

    let listener = thread::Builder::new()
        .name("TCPListener".to_string())
        .spawn(move || {
            listen_tcp(peer_arc_clone_listen);
        })
        .unwrap();
    let peer_arc_clone_interact = peer_arc.clone();

    //send request existing network table
    send_table_request(&ip_address, &own_addr, own_name);

    let interact = thread::Builder::new()
        .name("Interact".to_string())
        .spawn(move || {
            //spawn shell
            spawn_shell(peer_arc_clone_interact);
        })
        .unwrap();
    listener.join().expect_err("Could not join Listener");
    interact.join().expect_err("Could not join Interact");
    Ok(())
}

fn listen_tcp(arc: Arc<Mutex<Peer>>) -> Result<(), String> {
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
                handle_incoming_requests(deserialized, &mut peer);
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

fn handle_incoming_requests(request: SendRequest, peer: &mut Peer) {
    let copy = request.value;
    let value = match String::from_utf8(copy) {
        Ok(val) => val,
        Err(utf) => return
    };
    match request.action.as_ref() {
        "get_network_table" => {
            //@TODO check name in network table with hashmap contains key function
            println!("network table request with name: {}", value);
            send_network_table(request.from, &peer);
        }
        "ack_network_table" => {
            let networkTable = json_string_to_network_table(value);
            for (key, addr) in networkTable {
                peer.network_table.insert(key, addr);
            }
            dbg!(&peer.network_table);
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
