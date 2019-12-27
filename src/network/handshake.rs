use std::string::ToString;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::network::peer::Peer;
use crate::network::send_request::SendRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkInfo {
    name: String,
    address: String,
}

pub fn json_string_to_network_table(json_string: String) -> HashMap<String, SocketAddr> {
    let info_array: Vec<NetworkInfo> = match serde_json::from_str(json_string.as_str()) {
        Ok(val) => val,
        Err(e) => {
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

//request to get hashmap table
pub fn send_table_request(target: &SocketAddr, from: &SocketAddr, name: &str) {
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