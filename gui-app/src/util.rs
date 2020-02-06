use meff::utils::AppListener;
use meff::network;
use std::net::SocketAddr;
use meff::network::peer::Peer;
use meff::utils::Instructions::{GET, REMOVE};
use meff::network::{
    send_delete_peer_request, send_play_request, send_read_request, send_status_request,
    send_write_request, push_music_to_database,
};
use std::cell::RefCell;
use std::collections::HashMap;
use gtk::Widget;
use glib::{Sender, Receiver};
use meff::audio::MusicState::{CONTINUE, PAUSE, PLAY, STOP};
use meff::audio::MusicState;

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
        unimplemented!()
    }

    fn file_status_changed(&mut self, name: String, instr: String) {
        println!("new_file_saved");
        //@TODO remove unwrap
        self.sender.as_ref().unwrap().send((name, instr));
        //self.songs.push(name);
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
        let ip = self.peer.as_ref().unwrap().ip_address;
        send_read_request(ip, &title, REMOVE);
    }

    fn music_control(&mut self, instr: MusicState) {
        match self.cur_selected_song.as_ref() {
            Some(song) => {
                let ip = self.peer.as_ref().unwrap().ip_address;
                send_play_request(&song, ip, instr);
            }
            None => {

            }
        }
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
}

