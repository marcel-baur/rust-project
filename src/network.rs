use std::collections::HashMap;
use std::io::Bytes;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

extern crate get_if_addrs;

use super::database::*;

pub struct Peer {
    name: String,
    ip_address: SocketAddr,
    network_table: HashMap<String, SocketAddr>,
    database: Database, // TODO: use correct datatype for value
}

impl Peer {
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
}

pub fn create_network(onw_name: &str) -> Peer {
    let ifs = match get_if_addrs::get_if_addrs() {
        Ok(v) => v,
        Err(v) => panic!("Error while looking for ipaddress"),
    };
    let if_options = ifs
        .into_iter()
        .find(|i| i.name == "en0".to_string() && i.addr.ip().is_ipv4());
    let this_ipv4: String = if let Some(interface) = if_options {
        interface.addr.ip().to_string()
    } else {
        "Local ip address not found".to_string()
    };
    println!("Local IP Adress: {}", this_ipv4);
    let tokens: Vec<&str> = this_ipv4.split(".").collect();

    let peer_socket_addr = parse_socket_addr_from_string(tokens);
    let mut network_table = HashMap::new();
    network_table.insert(onw_name.to_string(), peer_socket_addr);
    Peer::create(peer_socket_addr, onw_name, network_table)
}

fn parse_socket_addr_from_string(tokens: Vec<&str>) -> SocketAddr {
    let mut int_tokens = Vec::new();
    for token in tokens {
        let int_token = match token.parse::<u8>() {
            Ok(val) => val,
            Err(e) => panic!("Parsing string to number failed"),
        };
        int_tokens.push(int_token);
    }
    SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(
            int_tokens[0],
            int_tokens[1],
            int_tokens[2],
            int_tokens[3],
        )),
        8080,
    )
}
