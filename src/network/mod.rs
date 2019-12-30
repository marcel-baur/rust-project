use std::io::Read;
use std::net::TcpListener;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

mod handshake;
pub mod peer;
mod response;
mod send_request;

extern crate get_if_addrs;
extern crate rand;
use rand::Rng;

use crate::network::handshake::{
    json_string_to_network_table, send_change_name_request, send_network_table, send_table_request,
    send_table_to_all_peers,
};
use crate::network::peer::{create_peer, Peer};
use crate::network::response::Message::DataStored;
use crate::network::response::*;
use crate::network::send_request::SendRequest;
use crate::shell::spawn_shell;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

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
        None => return Err("Failed to find any network address".to_string()),
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
                        dbg!(e); // missing field `value` at line 1 column 71
                        let error_message = format!("Could not deserialize {:?}", &buf);
                        return Err(error_message.to_string());
                    }
                };
                let mut peer = clone.lock().unwrap();
                //                dbg!(&deserialized);
                handle_incoming_requests(deserialized, &mut peer);
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
    let value_clone = request.value.clone();
    //    let copy = request.value;
    //    let value = match String::from_utf8(copy) {
    //        Ok(val) => val,
    //        Err(utf) => {
    //            dbg!(utf);
    //            return;
    //        }
    //    };
    println!("Handle incoming request {:?}", request.action);
    match request.action.as_ref() {
        "get_network_table" => {
            let copy = request.value;
            let value = match String::from_utf8(copy) {
                Ok(val) => val,
                Err(utf) => {
                    dbg!(utf);
                    return;
                }
            };
            // checks if key is unique, otherwise send change name request
            if peer.network_table.contains_key(&value) {
                let name = format!("{}+{}", &value, "1");
                send_change_name_request(request.from, peer.get_ip(), name.as_ref());
            } else {
                send_network_table(request.from, &peer);
            }
        }
        "ack_network_table" => {
            let copy = request.value;
            let value = match String::from_utf8(copy) {
                Ok(val) => val,
                Err(utf) => {
                    dbg!(utf);
                    return;
                }
            };
            let network_table = json_string_to_network_table(value);
            for (key, addr) in network_table {
                peer.network_table.insert(key, addr);
            }
            send_table_to_all_peers(peer);
        }
        "update_network_table" => {
            let copy = request.value;
            let value = match String::from_utf8(copy) {
                Ok(val) => val,
                Err(utf) => {
                    dbg!(utf);
                    return;
                }
            };
            let new_network_peer = json_string_to_network_table(value);
            for (key, addr) in new_network_peer {
                peer.network_table.insert(key, addr);
            }
            dbg!(&peer.network_table);
        }
        "change_name" => {
            let copy = request.value;
            let value = match String::from_utf8(copy) {
                Ok(val) => val,
                Err(utf) => {
                    dbg!(utf);
                    return;
                }
            };
            peer.network_table.remove(&peer.name);
            peer.name = value;
            peer.network_table
                .insert(peer.name.clone(), peer.ip_address);
            //send request existing network table
            send_table_request(
                &SocketAddr::from_str(&request.from).unwrap(),
                peer.get_ip(),
                &peer.name,
            );
        }
        "write" => {
            peer.process_store_request((request.key.clone(), value_clone.clone()));

            let redundant_target = other_random_target(&peer.network_table, peer.get_ip());
            dbg!(redundant_target);
            match redundant_target {
                Some(target) => {
                    send_write_request(
                        target,
                        *peer.get_ip(),
                        (request.key.clone(), value_clone.clone()),
                        true,
                    );
                }
                None => println!("Only peer in network. No redundancy possible"),
            };
            // TODO: Send response, fix address parse error
            dbg!(&request.from);
            match request.from.parse::<SocketAddr>() {
                Ok(target_address) => {
                    send_write_response(target_address, *peer.get_ip(), request.key.clone());
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
        "write_redundant" => {
            println!("Redundant write received");
            peer.process_store_request((request.key, value_clone));
        }
        "write_response" => {
            println!("Response: Successfully stored key {}", request.key);
        }
        _ => {
            println!("no valid request");
        }
    }
}

pub fn send_write_request(
    target: SocketAddr,
    origin: SocketAddr,
    data: (String, Vec<u8>),
    redundant: bool,
) {
    let mut stream = TcpStream::connect(target).unwrap();
    let mut action;
    if let true = redundant {
        println!("Redundant");
        action = "write_redundant";
    } else {
        println!("Initial");
        action = "write";
    }
    let buf = SendRequest {
        value: data.1,
        key: data.0,
        from: origin.to_string(), // TODO: IP vom sender, nicht vom ziel
        action: action.to_string(),
    };
    let serialized = match serde_json::to_writer(&stream, &buf) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &buf);
        }
    };
}

fn other_random_target(
    network_table: &HashMap<String, SocketAddr>,
    own_ip: &SocketAddr,
) -> Option<SocketAddr> {
    if network_table.len() == 1 {
        return None;
    }
    let mut rng = rand::thread_rng();
    let mut index = rng.gen_range(0, network_table.len());
    let mut target = network_table.values().skip(index).next().unwrap();
    while target == own_ip {
        index = rng.gen_range(0, network_table.len());
        target = network_table.values().skip(index).next().unwrap();
    }
    return Some(*target);
}

pub fn send_write_response(target: SocketAddr, origin: SocketAddr, key: String) {
    let mut stream = TcpStream::connect(target).unwrap();
    let mut value: Vec<u8> = Vec::new();
    value.push(0);
    let response = SendRequest {
        from: origin.to_string(),
        key,
        value,
        action: "write_response".to_string(),
    };
    let serialized = match serde_json::to_writer(&stream, &response) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize Response {:?}", &response);
        }
    };
}
