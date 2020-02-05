use meff::utils::AppListener;
use meff::network;
use std::net::SocketAddr;
use meff::network::peer::Peer;
use std::cell::RefCell;

//Music entertainment for friends application model
#[derive(Clone)]
pub struct MEFFM {
    pub songs: Vec<String>,
    pub peer: Option<Peer>,
}

impl AppListener for MEFFM {
    fn notify(&self) {
        println!("Hello world");
    }

    fn notify_status(&self, files: Vec<String>, name: String) {
        unimplemented!()
    }

    fn new_file_saved(&mut self, name: String) {
        //self.songs.push(name);
    }
}

impl MEFFM {
    pub fn new() -> MEFFM {
        let mut songs = Vec::new();
        MEFFM { songs, peer: None }
    }

    pub fn start(&mut self, name: String, port: String, ip: Option<SocketAddr>) {
        let peer = match network::startup(&name, &port, ip, Box::new(self.clone())) {
            Ok(p) => p,
            Err(_e) => {
                return;
            } // error!("Could not join network {:?}", e);
        };
        self.peer = Some(peer.lock().unwrap().clone());
    }

    pub fn push(&mut self) {

    }
}

