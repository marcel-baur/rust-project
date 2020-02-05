use meff::utils::AppListener;
use meff::network;
use std::net::SocketAddr;
use meff::network::peer::Peer;
use meff::network::{
    send_delete_peer_request, send_play_request, send_read_request, send_status_request,
    send_write_request, push_music_to_database,
};
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
        println!("new_file_saved");
        //self.songs.push(name);
    }
}

impl MEFFM {
    pub fn new() -> MEFFM {
        let mut songs = Vec::new();
        MEFFM { songs, peer: None }
    }

    //@TODO return result
    pub fn start(&mut self, name: String, port: String, ip: Option<SocketAddr>) {
        let peer = match network::startup(&name, &port, ip, Box::new(self.clone())) {
            Ok(p) => p,
            Err(_e) => {
                //@TODO exit programm
                return;
            } // error!("Could not join network {:?}", e);
        };
        let peer_unlock = peer.lock().unwrap();
        let mut peer_clone = peer_unlock.clone();
        self.peer = Some(peer_clone);
    }

    pub fn push(&mut self, path: String, title: String) {
        let ip = self.peer.as_ref().unwrap().ip_address;
        let mut peer_clone = self.peer.as_ref().unwrap().clone();
        match push_music_to_database(&title, &path, ip,  &mut peer_clone) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to push {} to database", path);
            }
        };
    }
}

