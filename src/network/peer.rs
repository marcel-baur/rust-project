use crate::database::Database;
use crate::network::get_own_ip_address;
use crate::utils::Instructions;
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
    pub database: Database,
    pub open_request_table: HashMap<SystemTime, Instructions>,
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
        open_request_table: HashMap<SystemTime, Instructions>,
    ) -> Peer {
        Peer {
            name: onw_name.to_string(),
            ip_address,
            network_table,
            database: Database::new(),
            //@TODO refactor this! we need a kind of request list
            open_request_table,
        }
    }

    pub fn get_ip(&self) -> &SocketAddr {
        &self.ip_address
    }

    pub fn get_db(&self) -> &Database {
        &self.database
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

    pub fn add_new_request(&mut self, time: &SystemTime, content: Instructions) {
        self.open_request_table.insert(*time, content);
    }

    pub fn delete_handled_request(&mut self, time: &SystemTime) {
        self.open_request_table.remove(time);
    }

    pub fn delete_file_from_database(&mut self, song_name: &str) {
        self.database.data.remove(song_name);
    }

    pub fn drop_peer_by_ip(&mut self, addr: &SocketAddr) {
        let tmp = self.network_table.clone();
        let dropped = tmp.iter().filter(|&(_, &v)| v == *addr).map(|(k, _)| k);
        for k in dropped {
            self.network_table.remove_entry(k);
        }
    }

    /// return the values of the network_table as a vector
    pub fn get_all_socketaddr_from_peers(&self) -> Vec<SocketAddr> {
        let values = self.network_table.values();
        let mut addresses = Vec::new();
        for val in values {
            addresses.push(*val);
        }
        addresses.sort_by(|a, b| a.port().cmp(&b.port()));
        return addresses;
    }

    /// returns the next four `SocketAddr` in the network_table
    pub fn get_heartbeat_successors(&mut self) -> Vec<SocketAddr> {
        let values = self.network_table.values();
        let mut addresses = Vec::new();
        for val in values {
            addresses.push(val);
        }
        addresses.sort_by(|a, b| a.port().cmp(&b.port()));
        let index = addresses.iter().position(|&r| r == self.get_ip()).unwrap();

        let mut successors = Vec::new();
        for i in index + 1..index + 4 {
            if i >= addresses.len() {
                let j = i - addresses.len();
                successors.push(**addresses.get(j).unwrap());
                continue;
            }
            successors.push(**addresses.get(i).unwrap());
        }
        return successors;
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
    let open_request_table = HashMap::new();
    let peer = Peer::create(
        peer_socket_addr,
        onw_name,
        network_table,
        open_request_table,
    );
    Ok(peer)
}
