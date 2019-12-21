use std::collections::HashMap;
use std::io::Bytes;
use std::net::SocketAddr;

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
        return Peer {
            name: onw_name.to_string(),
            ip_address,
            network_table,
            database: Database::new(),
        };
    }
}

pub fn create_network(onw_name: &str) -> Peer {
    todo!();
}
