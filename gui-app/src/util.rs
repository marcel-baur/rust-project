use meff::utils::AppListener;
use std::net::SocketAddr;
use meff::interface::{Peer, MusicState, start, music_request, upload_music, music_control, delete_peer, ListenerInstr};
use glib::{Sender};
use meff::interface::MusicState::{PAUSE, PLAY, STOP, CONTINUE};
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use gtk::AccelGroupExt;
use std::borrow::BorrowMut;
use gdk::enums::key::{p, e};
use meff::interface::Instructions::{REMOVE, GET};

//Music entertainment for friends application model
#[derive(Clone)]
pub struct Model {
    pub peer: Option<Arc<Mutex<Peer>>>,
    pub sender: Option<Sender<(String, ListenerInstr)>>,
    pub is_playing: Arc<Mutex<bool>>,
}

impl AppListener for Model {
    fn notify(&self) {
        println!("Hello world");
    }

    #[allow(unused_variables)]
    fn notify_status(&self, files: Vec<String>, name: String) {
        println!("Received status");
    }

    #[allow(unused_variables)]
    fn file_status_changed(&mut self, name: String, instr: ListenerInstr) {
        //@TODO remove unwrap
        match self.sender.as_ref().unwrap().send((name, instr)) {
            Ok(_) => println!("file status changed"),
            Err(_) => println!("file status changed error")
        }
    }

    #[allow(unused_variables)]
    fn player_playing(&mut self, title: Option<String>) {
        *self.is_playing.lock().unwrap() = true;
    }

    #[allow(unused_variables)]
    fn player_stopped(&mut self) {
        *self.is_playing.lock().unwrap() = false;
    }

}

impl Model {
    pub fn new() -> Model {
        Model {peer: None, sender: None, is_playing: Arc::new(Mutex::new(false))}
    }

    pub fn set_sender(&mut self, sender: Sender<(String, ListenerInstr)>) {
        self.sender = Some(sender);
    }

    pub fn start(&mut self, name: String, port: String, ip: Option<SocketAddr>) -> Result<(), String> {
        let clone = Box::new(self.clone());
        let peer = start(clone, name, port, ip);
        if peer.is_err() {
            return Err(peer.err().unwrap());
        }
        self.peer = Some(peer.ok().unwrap());
        Ok(())
    }

    pub fn push(&mut self, path: String, title: String) {
        let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
        let mut peer_clone = peer_unlock.clone();

        let ip = peer_clone.ip_address;
        match upload_music(&title, &path, ip,  &mut peer_clone) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Failed to push {} to database", path);
            }
        };
        drop(peer_unlock);
    }

    pub fn remove_title(&mut self, title: String) {
        let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
        let mut peer_clone = peer_unlock.clone();

        music_request(&mut peer_clone, &title, REMOVE);
        drop(peer_unlock);
    }

    fn music_control(&mut self, song: Option<String>, instr: MusicState) {
        let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
        let mut peer_clone = peer_unlock.clone();
        music_control(song, &mut peer_clone, instr);
        drop(peer_unlock);
    }

    pub fn status(&mut self) -> HashMap<String, SocketAddr> {
        let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
        let peer_clone = peer_unlock.clone();

        let list = peer_clone.network_table;
        drop(peer_unlock);
        list
    }

    pub fn stream(&mut self, search: String) {
        self.music_control(Some(search), PLAY);
    }

    pub fn download(&mut self, title: String) {
        let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
        let mut peer_clone = peer_unlock.clone();
        music_request(&mut peer_clone, &title, GET);
        drop(peer_unlock);
    }

    pub fn play(&mut self, title: Option<String>) {
        if *self.is_playing.lock().unwrap() {
            self.music_control(None,CONTINUE);
        } else {
            self.music_control(title, PLAY);
        }
    }

    pub fn pause(&mut self) {
        self.music_control(None,PAUSE);
    }

    pub fn stop(&mut self) {
        *self.is_playing.lock().unwrap() = false;
        self.music_control(None,STOP);
    }

    pub fn quit(&mut self) {
        let peer_unlock = self.peer.as_ref().unwrap().lock().unwrap();
        let mut peer_clone = peer_unlock.clone();

        delete_peer(&mut peer_clone);
        drop(peer_unlock);
    }
}

