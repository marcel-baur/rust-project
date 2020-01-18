use std::io::Read;
use std::net::TcpListener;
use std::net::{SocketAddr, TcpStream};
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

mod handshake;
mod notification;
pub mod peer;
mod response;

extern crate get_if_addrs;
extern crate rand;
use rand::Rng;

use crate::network::handshake::{
    json_string_to_network_table, send_change_name_request, send_network_table, send_table_request,
    send_table_to_all_peers, update_table_after_delete,
};
use crate::network::notification::Content::{ExitPeer, FindFile};
use crate::network::notification::*;
use crate::network::peer::{create_peer, Peer};
use crate::network::response::Message::DataStored;
use crate::network::response::*;
use crate::shell::{print_external_files, spawn_shell};
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use std::time::SystemTime;

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
        let mut buf = String::new();
        //        dbg!(&stream);
        match stream {
            Ok(mut s) => {
                s.read_to_string(&mut buf).unwrap();
                //                let deserialized: SendRequest = match serde_json::from_str(&buf) {
                //                    Ok(val) => val,
                //                    Err(e) => {
                //                        dbg!(e);
                //                        println!("Could not deserialize {:?}", &buf);
                //                        continue; // skip this stream
                //                    }
                //                };
                let des: Notification = match serde_json::from_str(&buf) {
                    Ok(val) => val,
                    Err(e) => {
                        dbg!(e);
                        println!("Could not deserialize {:?}", &buf);
                        continue; // skip this stream
                    }
                };
                let mut peer = clone.lock().unwrap();
                //                dbg!(&deserialized);
                handle_notification(des, &mut peer);
                //                handle_incoming_requests(deserialized, &mut peer);
                drop(peer);
                println!("Request handled.");
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

fn handle_incoming_response(response: Response, peer: &mut Peer) {
    return;
}

fn handle_notification(notification: Notification, peer: &mut Peer) {
    match notification.content {
        Content::PushToDB { key, value, from } => {
            peer.process_store_request((key.clone(), value.clone()));
            let redundant_target = other_random_target(&peer.network_table, peer.get_ip());
            match redundant_target {
                Some(target) => {
                    send_write_request(target, *peer.get_ip(), (key.clone(), value.clone()), true);
                }
                None => println!("Only peer in network. No redundancy possible"),
            };
            match from.parse::<SocketAddr>() {
                Ok(target_address) => {
                    send_write_response(target_address, *peer.get_ip(), key.clone());
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
        Content::RedundantPushToDB { key, value, from } => {
            peer.process_store_request((key.clone(), value.clone()));
        }
        Content::ChangePeerName { value, from } => {
            peer.network_table.remove(&peer.name);
            peer.name = value;
            peer.network_table
                .insert(peer.name.clone(), peer.ip_address);
            //send request existing network table
            send_table_request(
                &SocketAddr::from_str(&from.to_string()).unwrap(),
                peer.get_ip(),
                &peer.name,
            );
        }
        Content::SendNetworkTable { value, from } => {
            let table = match String::from_utf8(value) {
                Ok(val) => val,
                Err(utf) => {
                    dbg!(utf);
                    return;
                }
            };
            let network_table = json_string_to_network_table(table);
            for (key, addr) in network_table {
                peer.network_table.insert(key, addr);
            }
            send_table_to_all_peers(peer);
        }
        Content::SendNetworkUpdateTable { value, from } => {
            let table = match String::from_utf8(value) {
                Ok(val) => val,
                Err(utf) => {
                    dbg!(utf);
                    return;
                }
            };
            let new_network_peer = json_string_to_network_table(table);
            for (key, addr) in new_network_peer {
                peer.network_table.insert(key, addr);
            }
        }
        Content::RequestForTable { value, from } => {
            // checks if key is unique, otherwise send change name request
            if peer.network_table.contains_key(&value) {
                let name = format!("{}+{}", &value, "1");
                send_change_name_request(from.to_string(), peer.get_ip(), name.as_ref());
            } else {
                send_network_table(from.to_string(), &peer);
            }
        }
        Content::FindFile { key } => {
            let id = SystemTime::now();
            peer.add_new_request(&id, &key);

            for (_key, value) in &peer.network_table {
                read_file_exist(*value, &key, id.clone());
            }
        }
        Content::ExistFile { id, key, from } => {
            let exist = peer.does_file_exist(key.as_ref());
            send_exist_response(from, key.as_ref(), exist);
        }
        Content::Response { from, message } => {}
        Content::ExitPeer { addr } => {
            for (_key, value) in &peer.network_table {
                if *value != addr {
                    update_table_after_delete(*value, addr, &peer.name);
                }
            }
            process::exit(0);
        }
        Content::DeleteFromNetwork { name, from } => {
            if peer.network_table.contains_key(&name) {
                peer.network_table.remove(&name);
            }
        }
        Content::ExistFileResponse { key, from, exist } => {}
        Content::SelfStatusRequest {} => {
            for (_name, addr) in &peer.network_table {
                send_status_request(*addr, *peer.get_ip());
            }
        }
        Content::StatusRequest { from } => {
            let mut res: Vec<String> = Vec::new();
            for (k, _v) in &peer.get_db().data {
                res.push(k.to_string());
            }
            send_local_file_status(from, res, *peer.get_ip());
        }
        Content::StatusResponse { from, files } => {
            print_external_files(files);
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
        action = "write_redundant";
        let not = Notification {
            content: Content::RedundantPushToDB {
                key: data.0,
                value: data.1,
                from: origin.to_string(),
            },
            from: origin,
        };
        let serialized = match serde_json::to_writer(&stream, &not) {
            Ok(ser) => ser,
            Err(_e) => {
                println!("Failed to serialize SendRequest {:?}", &not);
            }
        };
    } else {
        action = "write";
        let not = Notification {
            content: Content::PushToDB {
                key: data.0,
                value: data.1,
                from: origin.to_string(),
            },
            from: origin,
        };
        let serialized = match serde_json::to_writer(&stream, &not) {
            Ok(ser) => ser,
            Err(_e) => {
                println!("Failed to serialize SendRequest {:?}", &not);
            }
        };
    }
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

    let not = Notification {
        content: Content::Response {
            from: origin,
            message: Message::DataStored { key },
        },
        from: origin,
    };
    let serialized = match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize Response {:?}", &not);
        }
    };
}

pub fn send_read_request(target: SocketAddr, name: &str) {
    /// Communicate to the listener that we want to find the location of a given file
    let mut stream = TcpStream::connect(target).unwrap();

    let not = Notification {
        content: Content::FindFile {
            key: name.to_string(),
        },
        from: target,
    };

    let serialized = match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

pub fn read_file_exist(target: SocketAddr, name: &str, id: SystemTime) {
    /// Sends a request to the other peers to check if they have the wanted file
    let mut stream = TcpStream::connect(target).unwrap();

    let not = Notification {
        content: Content::ExistFile {
            id,
            key: name.to_string(),
            from: target,
        },
        from: target,
    };

    let serialized = match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

pub fn send_exist_response(target: SocketAddr, name: &str, exist: bool) {
    //let mut stream = TcpStream::connect(target).unwrap();
}

pub fn send_delete_peer_request(target: SocketAddr) {
    let mut stream = TcpStream::connect(target).unwrap();

    let not = Notification {
        content: Content::ExitPeer { addr: target },
        from: target,
    };

    let serialized = match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

pub fn send_self_status_request(target: SocketAddr) {
    let mut stream = TcpStream::connect(target).unwrap();

    let not = Notification {
        content: Content::SelfStatusRequest {},
        from: target,
    };

    let serialized = match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

fn send_status_request(target: SocketAddr, from: SocketAddr) {
    let mut stream = TcpStream::connect(target).unwrap();

    let not = Notification {
        content: Content::StatusRequest { from },
        from,
    };

    let serialized = match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

fn send_local_file_status(target: SocketAddr, files: Vec<String>, from: SocketAddr) {
    let mut stream = TcpStream::connect(target).unwrap();
    let not = Notification {
        content: Content::StatusResponse { files, from },
        from,
    };

    let serialized = match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}
