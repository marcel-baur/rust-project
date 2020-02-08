use crate::interface::Peer;
use crate::network::send_read_request;
use crate::utils::Instructions::PLAY;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Cursor};
use std::string::ToString;
use std::fs;

pub struct MusicPlayer {
    is_playing: bool,
    current_song_name: Option<String>,
}

pub fn create_sink() -> Result<MusicPlayer, String> {
    println!("create_sink");
    Err("Server Version of MEFF-Library".to_string())
}

pub fn save_music_to_disk(music: Vec<u8>, name: &String) -> Result<(), String> {
    println!("{}", "save_music_to_disk".to_string());
    let path = format!("../file/{}.mp3", name);
    match fs::write(path ,music) {
        Ok(_) => return Ok(()),
        Err(_e) => return Err("could not save file to disk".to_string()),
    };
}


/// plays audio when mp3 is in database otherwise sends request to find file
/// # Arguments:
///
/// * `name` - String including mp3 name (key in our database)
///
pub fn play_music(peer: &mut Peer, name: &Option<String>, sink: &mut MusicPlayer) -> Result<(), String> {
    Err("Server Version of MEFF-Library".to_string())
}

pub fn pause_current_playing_music(sink: &mut MusicPlayer) -> Result<(), String> {
    Err("Server Version of MEFF-Library".to_string())
}

pub fn stop_current_playing_music(sink: &mut MusicPlayer) -> Result<(), String> {
    Err("Server Version of MEFF-Library".to_string())
}

pub fn continue_paused_music(sink: &mut MusicPlayer) -> Result<(), String> {
    Err("Server Version of MEFF-Library".to_string())
}

pub fn play_music_by_vec(music: Vec<u8>, sink: &mut MusicPlayer, name: String) -> Result<(), String> {
    Err("Server Version of MEFF-Library".to_string())
}
