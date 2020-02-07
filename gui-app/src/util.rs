use meff::utils::AppListener;
use meff::network;
use std::net::SocketAddr;
use meff::network::peer::Peer;
use meff::utils::Instructions::{REMOVE};
use meff::network::{send_delete_peer_request, send_play_request, send_read_request, push_music_to_database};
use glib::{Sender};
use meff::audio::MusicState::{PAUSE, PLAY, STOP};
use meff::audio::MusicState;
use std::collections::HashMap;

//Music entertainment for friends application model
#[derive(Clone)]
pub struct MEFFM {
    pub cur_selected_song: Option<String>,
    pub peer: Option<Peer>,
    pub sender: Option<Sender<(String, String)>>,
}

impl AppListener for MEFFM {
    fn notify(&self) {
        println!("Hello world");
    }

    fn notify_status(&self, files: Vec<String>, name: String) {
        println!("Received status");
    }

    fn file_status_changed(&mut self, name: String, instr: String) {
        println!("new_file_saved");
        //@TODO remove unwrap
        self.sender.as_ref().unwrap().send((name, instr));
    }
}

impl MEFFM {
    pub fn new() -> MEFFM {
        MEFFM { cur_selected_song: None, peer: None, sender: None}
    }

    pub fn set_sender(&mut self, sender: Sender<(String, String)>) {
        self.sender = Some(sender);
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

    pub fn remove_title(&mut self, title: String) {
        let mut peer = self.peer.as_ref().unwrap().clone();
        send_read_request(&mut peer, &title, REMOVE);
    }

    fn music_control(&mut self, instr: MusicState) {
        match self.cur_selected_song.as_ref() {
            Some(song) => {
                let mut peer_clone = self.peer.as_ref().unwrap().clone();
                send_play_request(&song, &mut peer_clone, instr);
            }
            None => {

            }
        }
    }

    pub fn status(&mut self) {
        //send_status(&mut self.peer.as_ref().unwrap().clone())
    }

    pub fn play(&mut self) {
        self.music_control(PLAY);
    }

    pub fn pause(&mut self) {
        self.music_control(PAUSE);
    }

    pub fn stop(&mut self) {
        self.music_control(STOP);
    }

    pub fn get(&mut self) {
        
    }

    pub fn quit(&mut self) {
        let mut peer = self.peer.as_ref().unwrap().clone();
        send_delete_peer_request(&mut peer);
    }
}

