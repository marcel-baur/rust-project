use std::io::{Read, ErrorKind};
use std::net::TcpListener;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::{thread, io, fs};

mod handshake;
mod music_exchange;
mod notification;
pub mod peer;
mod request;
mod response;

extern crate get_if_addrs;
extern crate rand;
use rand::Rng;

use crate::audio::{
    continue_paused_music, create_sink, pause_current_playing_music, play_music,
    stop_current_playing_music, MusicPlayer, MusicState,
};

use crate::utils::{AppListener, Instructions, HEARTBEAT_SLEEP_DURATION};
use handshake::send_table_request;
use notification::*;
use peer::{create_peer, Peer};
use request::{
    change_peer_name, delete_file_request, delete_from_network, dropped_peer, exist_file,
    exist_file_response, exit_peer, find_file, get_file, get_file_response, order_song_request,
    push_to_db, redundant_push_to_db, request_for_table, self_status_request, send_network_table,
    send_network_update_table, status_request,
};
use response::*;
use rodio::Sink;
use std::collections::HashMap;
use std::path::Path;

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
        Err(_e) => return Err("Could not parse ip address to SocketAddr".to_string()),
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

/// Create or join a network, depending on the value of `ip_address`. If the value is `None`, a new
/// network will be created. Otherwise the library will attempt to join an existing network on that
/// IP address.
/// # Parameters
/// `own_name` - name of the local peer
///
/// `port` - port on which the local peer will listen on
///
/// `ip_address` - IP Address of one of the peers of an existing network / `None` if a new network
/// is to be created
///
/// `app` - listener object of the application that implements the library.
/// # Returns
/// the peer object wrapped in a Mutex
pub fn startup(
    own_name: &str,
    port: &str,
    ip_address: Option<SocketAddr>,
    app: Box<dyn AppListener + Sync>,
) -> Result<Arc<Mutex<Peer>>, String> {
    let peer = create_peer(own_name, port).unwrap();
    let own_addr = peer.ip_address;
    let peer_arc = Arc::new(Mutex::new(peer));
    let peer_arc_clone_listen = peer_arc.clone();
    let peer_arc_clone_return = peer_arc.clone();
    let app_arc = Arc::new(Mutex::new(app));

    let listener = thread::Builder::new()
        .name("TCPListener".to_string())
        .spawn(move || {
            match listen_tcp(peer_arc_clone_listen, app_arc) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Failed to spawn listener");
                }
            };
        })
        .unwrap();
    let peer_arc_clone_interact = peer_arc.clone();
    let peer_arc_clone_heartbeat = peer_arc.clone();

    //send request existing network table
    match ip_address {
        Some(ip) => {
            send_table_request(ip, own_addr, own_name);
        }
        None => {
            println!("Ip address is empty");
        }
    }

//    let interact = thread::Builder::new()
//        .name("Interact".to_string())
//        .spawn(move || {
//            //spawn shell
//            match spawn_shell(peer_arc_clone_interact) {
//                Ok(_) => {}
//                Err(_) => {
//                    eprintln!("Failed to spawn shell");
//                }
//            };
//        })
//        .unwrap();
    let heartbeat = thread::Builder::new()
        .name("Heartbeat".to_string())
        .spawn(move || match start_heartbeat(peer_arc_clone_heartbeat) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Failed to spawn shell");
            }
        })
        .unwrap();
    return Ok(peer_arc_clone_return);
//    listener.join().expect_err("Could not join Listener");
//    heartbeat.join().expect_err("Could not join Heartbeat");

}

fn listen_tcp(arc: Arc<Mutex<Peer>>, app: Arc<Mutex<Box<dyn AppListener + Sync>>>) -> Result<(), String> {
    let clone = arc.clone();
    //let app_clone = app.clone();
    let listen_ip = clone.lock().unwrap().ip_address;
    let listener = TcpListener::bind(&listen_ip).unwrap();
    let mut sink = create_sink().unwrap();
    println!("Listening on {}", listen_ip);
    for stream in listener.incoming() {
        let mut buf = String::new();
        //        dbg!(&stream);
        match stream {
            Ok(mut s) => {
                s.read_to_string(&mut buf).unwrap();
                let des: Notification = match serde_json::from_str(&buf) {
                    Ok(val) => val,
                    Err(e) => {
                        dbg!(e);
                        println!("Could not deserialize {:?}", &buf);
                        continue; // skip this stream
                    }
                };
                let mut peer = clone.lock().unwrap();
                let mut app = app.lock().unwrap();

                handle_notification(des, &mut peer, &mut sink, &mut app);
                drop(peer);
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

/// starts the heartbeat
fn start_heartbeat(arc: Arc<Mutex<Peer>>) -> Result<(), String> {
    loop {
        thread::sleep(HEARTBEAT_SLEEP_DURATION);
        let peer = arc.lock().unwrap();
        let mut peer_clone = peer.clone();
        drop(peer);
        let network_size = peer_clone.network_table.len();
        if network_size == 1 {
            continue;
        } else if network_size <= 10 {
            send_heartbeat(&peer_clone.get_all_socketaddr_from_peers(), &mut peer_clone);
        } else {
            let successors = peer_clone.get_heartbeat_successors();
            send_heartbeat(&successors, &mut peer_clone);
        }
    }
}

/// send the heartbeat request to all targets in `targets`
fn send_heartbeat(targets: &[SocketAddr], peer: &mut Peer) {
    let mut cloned_peer = peer.clone();
    for addr in targets {
        let stream = match TcpStream::connect(addr) {
            Ok(s) => s,
            Err(_e) => {
                handle_lost_connection(*addr, &mut cloned_peer);
                return;
            }
        };
        let not = Notification {
            content: Content::Heartbeat,
            from: *cloned_peer.get_ip(),
        };
        match serde_json::to_writer(&stream, &not) {
            Ok(ser) => ser,
            Err(_e) => {
                println!("Failed to serialize SendRequest {:?}", &not);
            }
        };
    }
}

fn handle_notification(
    notification: Notification,
    peer: &mut Peer,
    sink: &mut MusicPlayer,
    listener: &mut Box<dyn AppListener + Sync>,
) {
    dbg!(&notification);
    let sender = notification.from;
    match notification.content {
        Content::PushToDB { key, value, from } => {
            push_to_db(key, value, from, peer);
        }
        Content::RedundantPushToDB { key, value, .. } => {
            redundant_push_to_db(key, value, peer);
        }
        Content::ChangePeerName { value } => {
            change_peer_name(value, sender, peer);
        }
        Content::SendNetworkTable { value } => {
            send_network_table(value, peer);
        }
        Content::SendNetworkUpdateTable { value } => {
            send_network_update_table(value, peer);
        }
        Content::RequestForTable { value } => {
            request_for_table(value, sender, peer);
        }
        Content::FindFile { song_name, instr } => {
            find_file(instr, song_name, peer);
        }
        Content::ExistFile { song_name, id } => {
            exist_file(song_name, id, sender, peer);
        }
        Content::ExistFileResponse { song_name, id } => {
            exist_file_response(song_name, id, sender, peer);
        }
        Content::GetFile { key, instr } => {
            get_file(instr, key, sender, peer);
        }
        Content::GetFileResponse { value, instr, key } => {
            get_file_response(instr, key, value, peer, sink);
        }
        Content::DeleteFileRequest { song_name } => {
            delete_file_request(song_name, peer);
        }
        Content::Response { .. } => {}
        Content::ExitPeer { addr } => {
            exit_peer(addr, peer);
        }
        Content::OrderSongRequest { song_name } => {
            order_song_request(song_name, peer);
        }
        Content::DeleteFromNetwork { name } => {
            delete_from_network(name, peer);
        }
        Content::SelfStatusRequest {} => {
            self_status_request(peer);
        }
        Content::StatusRequest {} => {
            status_request(sender, peer);
        }
        Content::StatusResponse { files, name } => {
            listener.notify_status(files, name);
        }
        Content::PlayAudioRequest { name, state } => {
            match state {
                MusicState::PLAY => play_music(peer, name.as_str(), sink),
                MusicState::PAUSE => pause_current_playing_music(sink),
                MusicState::STOP => stop_current_playing_music(sink),
                MusicState::CONTINUE => continue_paused_music(sink),
            };
        }
        Content::DroppedPeer { addr } => {
            dropped_peer(addr, peer);
        }
        Content::Heartbeat => {}
        Content::NewFileSaved {song_name} => {
            listener.new_file_saved(song_name);
        }
    }
}

pub fn send_write_request(
    target: SocketAddr,
    origin: SocketAddr,
    data: (String, Vec<u8>),
    redundant: bool,
    peer: &mut Peer,
) {
    let stream = match TcpStream::connect(target) {
        Ok(s) => s,
        Err(_e) => {
            handle_lost_connection(target, peer);
            return;
        }
    };
    if let true = redundant {
        let not = Notification {
            content: Content::RedundantPushToDB {
                key: data.0,
                value: data.1,
                from: origin.to_string(),
            },
            from: origin,
        };
        match serde_json::to_writer(&stream, &not) {
            Ok(ser) => ser,
            Err(e) => {
                error!("Could not serialize {:?}, Error: {:?}", &not, e);
                println!("Failed to serialize SendRequest {:?}", &not);
            }
        };
    } else {
        let not = Notification {
            content: Content::PushToDB {
                key: data.0,
                value: data.1,
                from: origin.to_string(),
            },
            from: origin,
        };
        match serde_json::to_writer(&stream, &not) {
            Ok(_ser) => {}
            Err(e) => {
                error!("Could not serialize {:?}, Error: {:?}", &not, e);
                println!("Failed to serialize SendRequest {:?}", &not);
            }
        };
    }
}

/// Selects a random `SocketAddr` from the `network_table` that is not equal to `own_ip`. Returns
/// `None` if there is no other `SocketAddr` in `network_table`.
fn other_random_target(
    network_table: &HashMap<String, SocketAddr>,
    own_ip: &SocketAddr,
) -> Option<SocketAddr> {
    if network_table.len() == 1 {
        return None;
    }
    let mut rng = rand::thread_rng();
    let mut index = rng.gen_range(0, network_table.len());
    let mut target = network_table.values().nth(index).unwrap();
    while target == own_ip {
        index = rng.gen_range(0, network_table.len());
        target = network_table.values().nth(index).unwrap();
    }
    Some(*target)
}

pub fn send_write_response(target: SocketAddr, origin: SocketAddr, key: String, peer: &mut Peer) {
    let stream = match TcpStream::connect(target) {
        Ok(s) => s,
        Err(_e) => {
            handle_lost_connection(target, peer);
            return;
        }
    };

    let not = Notification {
        content: Content::NewFileSaved {
            song_name: key.to_string(),
        },
        from: origin,
    };
    match serde_json::to_writer(&stream, &not) {
        Ok(_ser) => {}
        Err(e) => {
            error!("Could not serialize {:?}, Error: {:?}", &not, e);
            println!("Failed to serialize Response {:?}", &not);
        }
    };
}

/// Communicate to the listener that we want to find the location of a given file
pub fn send_read_request(target: SocketAddr, name: &str, instr: Instructions) {
    let not = Notification {
        content: Content::FindFile {
            instr,
            song_name: name.to_string(),
        },
        from: target,
    };

    tcp_request_with_notification(target, not);
}

pub fn send_delete_peer_request(target: SocketAddr) {
    let not = Notification {
        content: Content::ExitPeer { addr: target },
        from: target,
    };

    tcp_request_with_notification(target, not);
}

pub fn send_status_request(target: SocketAddr, from: SocketAddr, peer: &mut Peer) {
    let stream = match TcpStream::connect(target) {
        Ok(s) => s,
        Err(_e) => {
            handle_lost_connection(target, peer);
            return;
        }
    };

    let not = Notification {
        content: Content::StatusRequest {},
        from,
    };

    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

fn send_local_file_status(
    target: SocketAddr,
    files: Vec<String>,
    from: SocketAddr,
    peer_name: String,
) {
    let not = Notification {
        content: Content::StatusResponse {
            files,
            name: peer_name,
        },
        from,
    };

    tcp_request_with_notification(target, not);
}

pub fn send_play_request(name: &str, from: SocketAddr, state: MusicState) {
    let stream = match TcpStream::connect(from) {
        Ok(s) => s,
        Err(_e) => {
            //            handle_lost_connection(from, peer); TODO
            return;
        }
    };
    let not = Notification {
        content: Content::PlayAudioRequest {
            name: name.to_string(),
            state,
        },
        from,
    };
    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

fn handle_lost_connection(addr: SocketAddr, peer: &mut Peer) {
    //    peer.drop_peer_by_ip(&addr);
    let mut cloned_peer = peer.clone();
    // TODO: Send notification to other peers that this peer was dropped
    for other_addr in peer.network_table.values() {
        if *other_addr != addr {
            send_dropped_peer_notification(*other_addr, addr, &mut cloned_peer)
        }
    }
}

/// Send a notification to the peer at `target` that the peer at `dropped_addr` has left the network
/// or was dropped.
/// # Parameters:
/// - `target`: `SocketAddr` of the Peer that should receive the notification
/// - `dropped_addr`: `SocketAddr` of the Peer that is not connected anymore
/// - `peer`: the local `Peer`
fn send_dropped_peer_notification(target: SocketAddr, dropped_addr: SocketAddr, peer: &mut Peer) {
    let stream = match TcpStream::connect(target) {
        Ok(s) => s,
        Err(_e) => {
            handle_lost_connection(target, peer);
            return;
        }
    };
    let not = Notification {
        content: Content::DroppedPeer { addr: dropped_addr },
        from: *peer.get_ip(),
    };
    if let Err(_e) = serde_json::to_writer(&stream, &not) {
        println!("Failed to serialize SendRequest {:?}", &not);
    }
}

/// Send a notification to the peer at `target` that a new file has been saved.
/// # Parameters:
/// - `target`: `SocketAddr` of the Peer that should receive the notification
/// - `file_name`: name of the file as string
/// - `peer`: the local `Peer`
pub fn send_new_file_notification (target: SocketAddr, file_name: &str, peer: &mut Peer) {
    let stream = match TcpStream::connect(target) {
        Ok(s) => s,
        Err(_e) => {
            handle_lost_connection(target, peer);
            return;
        }
    };
    let not = Notification {
        content: Content::NewFileSaved {song_name: file_name.to_string()},
        from: *peer.get_ip(),
    };
    if let Err(_e) = serde_json::to_writer(&stream, &not) {
        println!("Failed to serialize SendRequest {:?}", &not);
    }
}

/// Function to check file path to mp3 and saves to db afterwards
/// # Arguments:
///
/// * `name` - String including mp3 name (key in our database)
/// * `file_path` - Path to the mp3 file
/// * `peer` - Peer
///
/// # Returns:
/// Result //@TODO
pub fn push_music_to_database(
    name: &str,
    file_path: &str,
    addr: SocketAddr,
    peer: &mut Peer,
) -> Result<(), io::Error> {
    // get mp3 file
    let path = Path::new(file_path);
    if path.exists() {
        let read_result = fs::read(path);
        match read_result {
            Ok(content) => {
                println!("Pushing... This can take a while");
                //@TODO save to database
                //                peer.get_db().add_file(name, content);
                //                peer.store((name.parse().unwrap(), content));
                send_write_request(addr, addr, (name.to_string(), content), false, peer);
                return Ok(());
            }
            Err(err) => {
                println!("Error while parsing file");
                return Err(err);
            }
        }
    } else {
        println!("The file could not be found at this path: {:?}", path);
    }
    Err(io::Error::new(ErrorKind::NotFound, "File Path not found!"))
}