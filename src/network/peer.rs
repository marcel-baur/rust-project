use crate::database::Database;
use crate::network::get_own_ip_address;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::string::ToString;
use std::time::SystemTime;

/// Represents a Peer in the network
#[derive(Clone)]
pub struct Peer {
    pub name: String,
    pub ip_address: SocketAddr,
    pub network_table: HashMap<String, SocketAddr>,
    database: Database,
    pub open_request_table: HashMap<SystemTime, String>,
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
        open_request_table: HashMap<SystemTime, String>,
    ) -> Peer {
        Peer {
            name: onw_name.to_string(),
            ip_address,
            network_table,
            database: Database::new(),
            open_request_table,
        }
    }

    pub fn store(&self, data: (String, Vec<u8>)) {
        let k = data.0;
        let v = data.1;
        //        self.database.add_file(&k, v);
    }

    pub fn get_ip(&self) -> &SocketAddr {
        return &self.ip_address;
    }

    pub fn get_db(&self) -> &Database {
        return &self.database;
    }

    pub fn process_store_request(&mut self, data: (String, Vec<u8>)) {
        self.database.data.insert(data.0, data.1);
    }

    pub fn find_file(&self, name: &str) -> Option<&Vec<u8>> {
        self.database.data.get(name)
    }

    pub fn does_file_exist(&self, name: &str) -> bool {
        match self.database.data.get(name) {
            Some(_t) => true,
            None => false,
        }
    }

    pub fn add_new_request(&mut self, time: &SystemTime, content: &str) {
        self.open_request_table.insert(*time, content.to_string());
    }

    pub fn check_request_still_active(&self, time: &SystemTime) -> bool {
        return self.open_request_table.contains_key(time);
    }

    pub fn delete_handled_request(&mut self, time: &SystemTime) {
        self.open_request_table.remove(time);
    }
}

/// Function to create a new network
/// # Arguments:
///
/// * `own_name` - String that denotes the name of the initial Peer
///
/// # Returns:
/// A new `Peer` if successful, error string if failed
pub fn create_peer(onw_name: &str, port: &str) -> Result<Peer, String> {
    let peer_socket_addr = match get_own_ip_address(port) {
        Ok(val) => val,
        Err(error_message) => return Err(error_message),
    };
    let mut network_table = HashMap::new();
    network_table.insert(onw_name.to_string(), peer_socket_addr);
    let mut open_request_table = HashMap::new();
    let peer = Peer::create(
        peer_socket_addr,
        onw_name,
        network_table,
        open_request_table,
    );
    Ok(peer)
}
