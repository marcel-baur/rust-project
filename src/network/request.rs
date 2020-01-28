use crate::network::{other_random_target, send_write_request, send_write_response, send_read_request, send_status_request, send_local_file_status};
use std::net::SocketAddr;
use crate::network::peer::Peer;
use crate::network::handshake::{send_table_request, json_string_to_network_table, send_table_to_all_peers, send_change_name_request, send_network_table_request, update_table_after_delete};
use crate::utils::Instructions;
use std::time::SystemTime;
use crate::network::music_exchange::{delete_redundant_song_request, read_file_exist, send_exist_response, send_file_request, send_get_file_reponse, song_order_request};
use crate::utils::Instructions::{PLAY, REMOVE, GET, ORDER};
use crate::audio::play_music_by_vec;
use std::process;

pub fn push_to_db(key: String, value: Vec<u8>, from: String, peer: &mut Peer) {
    if peer.database.data.contains_key(&key) {
        println!("File already exists in your database");
    } else {
        peer.process_store_request((key.clone(), value.clone()));
        println!("Saved file to database");

        let redundant_target = other_random_target(&peer.network_table, peer.get_ip());
        match redundant_target {
            Some(target) => {
                send_write_request(
                    target,
                    *peer.get_ip(),
                    (key.clone(), value.clone()),
                    true,
                    peer,
                );
            }
            None => println!("Only peer in network. No redundancy possible"),
        };
        match from.parse::<SocketAddr>() {
            Ok(target_address) => {
                send_write_response(target_address, *peer.get_ip(), key.clone(), peer);
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
}

pub fn redundant_push_to_db(key: String, value: Vec<u8>, peer: &mut Peer) {
    peer.process_store_request((key, value));
}

pub fn change_peer_name(value: String, sender: SocketAddr, peer: &mut Peer) {
    peer.network_table.remove(&peer.name);
    peer.name = value;
    peer.network_table
        .insert(peer.name.clone(), peer.ip_address);
    //send request existing network table
    send_table_request(sender, *peer.get_ip(), &peer.name);
}

pub fn send_network_table(value: Vec<u8>, peer: &mut Peer) {
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


pub fn send_network_update_table(value: Vec<u8>, peer: &mut Peer) {

    let table = match String::from_utf8(value) {
        Ok(val) => val,
        Err(utf) => {
            dbg!(utf);
            return;
        }
    };
    let new_network_peer = json_string_to_network_table(table);
    for (key, addr) in new_network_peer {
        let name = key.clone();
        peer.network_table.insert(key, addr);
        println!("{} joined the network.", name);
    }
}


pub fn request_for_table(value: String, sender: SocketAddr, peer: &mut Peer) {
    // checks if key is unique, otherwise send change name request
    if peer.network_table.contains_key(&value) {
        let name = format!("{}+{}", &value, "1");
        send_change_name_request(sender, *peer.get_ip(), name.as_ref());
    } else {
        send_network_table_request(sender, &peer);
    }
}

pub fn find_file(instr: Instructions, song_name: String, peer: &mut Peer) {
// @TODO there is no feedback when audio does not exist in "global" database (there is only the existsFile response, when file exists in database? change?
    // @TODO in this case we need to remove the request?
    if peer.get_db().get_data().contains_key(&song_name) {
        if instr == REMOVE {
            peer.delete_file_from_database(&song_name);
            println!("Remove file {} from database", &song_name);

            let id = SystemTime::now();
            peer.add_new_request(&id, instr);

            for (_key, value) in &peer.network_table {
                if _key != &peer.name {
                    delete_redundant_song_request(*value, peer.ip_address, &song_name);
                }
            }
        } else if instr == PLAY {
            // TODO: play music if file in own database
        }
    } else {
        let id = SystemTime::now();
        peer.add_new_request(&id, instr);

        for (_key, value) in &peer.network_table {
            if _key != &peer.name {
                read_file_exist(*value, peer.ip_address, &song_name, id);
            }
        }
    }
}

pub fn get_file(instr: Instructions, key: String, sender: SocketAddr, peer: &mut Peer) {
    match peer.find_file(key.as_ref()) {
        Some(music) => send_get_file_reponse(
            sender,
            peer.ip_address,
            key.as_ref(),
            music.clone(),
            instr,
        ),
        None => {
            //@TODO error handling}
            println!("TODO!");
        }
    }
}

pub fn get_file_response(instr: Instructions, key: String, value: Vec<u8>, peer: &mut Peer) {
    match instr {
        PLAY => {
            //save to tmp and play audio
            match play_music_by_vec(&value) {
                Ok(_) => {}
                Err(_) => {
                    println!("Could not play the requested file {}", &key);
                    error!("Failed to play music from {}", &key);
                }
            };
        },
        GET => {
            //Download mp3 file
        },
        ORDER => {
            peer.process_store_request((key.clone(), value.clone()));
        },
        _ => {}
    }
}

pub fn exist_file(song_name: String, id: SystemTime, sender:SocketAddr, peer: &mut Peer) {
    let exist = peer.does_file_exist(song_name.as_ref());
    if exist {
        send_exist_response(sender, peer.ip_address, song_name.as_ref(), id);
    }
}

pub fn exit_peer(addr: SocketAddr, peer: &mut Peer) {
    for value in peer.network_table.values() {
        if *value != addr {
            update_table_after_delete(*value, addr, &peer.name);
        }
    }
    let database = peer.get_db().get_data();
    let network_table = &peer.network_table;
    if network_table.len() > 1 {
        for (song, _value) in database {
            let redundant_target =
                other_random_target(network_table, peer.get_ip()).unwrap();
            song_order_request(redundant_target, peer.ip_address, song.to_string());
        }
    }
    process::exit(0);
}

pub fn delete_from_network(name: String, peer: &mut Peer) {
    if peer.network_table.contains_key(&name) {
        peer.network_table.remove(&name);
        println!("{} left the network.", &name);
    }
}

pub fn exist_file_response(song_name: String, id: SystemTime, sender: SocketAddr, peer: &mut Peer) {
    //Check if peer request is still active. when true remove it
    let peer_clone = peer.open_request_table.clone();
    match peer_clone.get(&id) {
        Some(instr) => {
            peer.delete_handled_request(&id);
            send_file_request(sender, peer.ip_address, song_name.as_ref(), instr.clone());
        }
        None => {
            println!("There is no file \"{}\" to remove.", &song_name);
        }
    }
}

pub fn status_request(sender: SocketAddr, peer: &mut Peer) {
    let mut res: Vec<String> = Vec::new();
    for k in peer.get_db().data.keys() {
        res.push(k.to_string());
    }
    let peer_name = &peer.name;
    send_local_file_status(sender, res, *peer.get_ip(), peer_name.to_string());
}

pub fn self_status_request(peer: &mut Peer) {
    let mut cloned_peer = peer.clone();
    for addr in peer.network_table.values() {
        send_status_request(*addr, *peer.get_ip(), &mut cloned_peer);
    }
}

pub fn dropped_peer(addr: SocketAddr, peer: &mut Peer) {
    println!("Peer at {:?} was dropped", addr);
    peer.drop_peer_by_ip(&addr);
}


pub fn order_song_request(song_name: String, peer: &mut Peer) {
    let network_table = &peer.network_table;
    // TODO: REVIEW unwrap
    if peer.get_db().get_data().contains_key(&song_name) {
        let redundant_target = other_random_target(network_table, peer.get_ip()).unwrap();
        song_order_request(redundant_target, peer.ip_address, song_name.to_string());
    } else {
        send_read_request(peer.ip_address, &song_name, Instructions::ORDER)
    }
}

pub fn delete_file_request(song_name: String, peer: &mut Peer) {
    if peer.database.data.contains_key(&song_name) {
        println!("Remove file {} from database", &song_name);
        peer.delete_file_from_database(&song_name);
    }
}