use crate::network::notification::*;
use crate::network::peer::Peer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use std::string::ToString;

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkInfo {
    name: String,
    address: String,
}

pub fn json_string_to_network_table(json_string: String) -> HashMap<String, SocketAddr> {
    let info_array: Vec<NetworkInfo> = match serde_json::from_str(json_string.as_str()) {
        Ok(val) => val,
        Err(_e) => {
            println!("no parcing hashmap");
            return HashMap::new();
        }
    };
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
        array.push(NetworkInfo {
            name: key.clone(),
            address: address.clone().to_string(),
        });
    }
    serde_json::to_string(&array).unwrap()
}

pub fn send_network_table(target: String, peer: &Peer) {
    let stream = TcpStream::connect(target).unwrap();
    let not = Notification {
        content: Content::SendNetworkTable {
            value: network_table_to_json(&peer.network_table).into_bytes(),
        },
        from: peer.ip_address,
    };
    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

pub fn send_network_update_table(
    target: String,
    from: String,
    hashmap: &HashMap<String, SocketAddr>,
) {
    //@TODO if connection error remove from network_table
    let stream = match TcpStream::connect(target) {
        Ok(stream) => stream,
        Err(_) => return,
    };
    let not = Notification {
        content: Content::SendNetworkUpdateTable {
            value: network_table_to_json(hashmap).into_bytes(),
        },
        from: from.parse::<SocketAddr>().unwrap(),
    };
    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

pub fn send_change_name_request(target: String, from: &SocketAddr, name: &str) {
    let stream = TcpStream::connect(target).unwrap();
    let not = Notification {
        content: Content::ChangePeerName {
            value: name.to_string(),
        },
        from: *from,
    };
    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SimpleRequest {:?}", &not);
        }
    };
}

pub fn send_table_to_all_peers(peer: &Peer) {
    let mut hashmap: HashMap<String, SocketAddr> = HashMap::new();
    hashmap.insert(peer.name.to_string(), peer.ip_address);

    let network_table = peer.network_table.clone();
    for (key, value) in network_table {
        // just update all other peers
        if key != peer.name {
            send_network_update_table(value.to_string(), peer.ip_address.to_string(), &hashmap);
        }
    }
}

//request to get hashmap table
pub fn send_table_request(target: &SocketAddr, from: &SocketAddr, name: &str) {
    let stream = match TcpStream::connect(target) {
        Ok(stream) => stream,
        Err(e) => {
            //@TODO this means ip-address is not right! add proper error handling
            dbg!(e);
            return;
        }
    };
    let not = Notification {
        content: Content::RequestForTable {
            value: name.to_string(),
        },
        from: *from,
    };
    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SimpleRequest {:?}", &not);
        }
    };
}

pub fn update_table_after_delete(target: SocketAddr, from: SocketAddr, name: &str) {
    let not = Notification {
        content: Content::DeleteFromNetwork {
            name: name.to_string(),
        },
        from,
    };

    tcp_request_with_notification(target, not);
}
