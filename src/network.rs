use std::collections::HashMap;
use std::io::Bytes;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

extern crate get_if_addrs;

use super::database::*;

/// Represents a Peer in the network
pub struct Peer {
    name: String,
    ip_address: SocketAddr,
    network_table: HashMap<String, SocketAddr>,
    database: Database, // TODO: use correct datatype for value
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
}

/// Function to create a new network
/// # Arguments:
///
/// * `own_name` - String that denotes the name of the initial Peer
///
/// # Returns:
/// A new `Peer`
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
    println!("Local IP Address: {}", this_ipv4);
    let tokens: Vec<&str> = this_ipv4.split(".").collect();
    let ipv4_port = format!("{}:{}", this_ipv4, "8080");
    let peer_socket_addr = match ipv4_port.parse::<SocketAddr>() {
        Ok(val) => val,
        Err(e) => panic!("Could not parse ip address to SocketAddr"),
    };
    let mut network_table = HashMap::new();
    network_table.insert(onw_name.to_string(), peer_socket_addr);
    Peer::create(peer_socket_addr, onw_name, network_table)
}
