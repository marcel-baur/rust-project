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
use std::sync::{Mutex, Arc};
use gtk::AccelGroupExt;

//Music entertainment for friends application model
#[derive(Clone)]
pub struct MEFFM {
    pub cur_selected_song: Option<String>,
    pub peer: Option<Arc<Mutex<Peer>>>,
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
        self.peer = Some(peer);
    }

    pub fn push(&mut self, path: String, title: String) {
        let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
        let mut peer_clone = peer_unlock.clone();

        let ip = peer_clone.ip_address;
        match push_music_to_database(&title, &path, ip,  &mut peer_clone) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to push {} to database", path);
            }
        };
        drop(peer_unlock);
    }

    pub fn remove_title(&mut self, title: String) {
        let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
        let mut peer_clone = peer_unlock.clone();

        send_read_request(&mut peer_clone, &title, REMOVE);
        drop(peer_unlock);
    }

    fn music_control(&mut self, instr: MusicState) {
        match self.cur_selected_song.as_ref() {
            Some(song) => {
                let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
                let mut peer_clone = peer_unlock.clone();

                send_play_request(&song, &mut peer_clone, instr);
                drop(peer_unlock);
            }
            None => {

            }
        }
    }

    pub fn status(&mut self) -> HashMap<String, SocketAddr> {
        let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
        let mut peer_clone = peer_unlock.clone();

        let list = peer_clone.network_table;
        drop(peer_unlock);
        list
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
        let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
        let mut peer_clone = peer_unlock.clone();

        send_delete_peer_request(&mut peer_clone);
        drop(peer_unlock);
    }
}

