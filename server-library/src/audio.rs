use crate::interface::Peer;
use crate::network::send_read_request;
use crate::utils::FileInstructions::PLAY;
use serde::{Deserialize, Serialize};
use std::io::{BufReader, Cursor};
use std::string::ToString;
use std::fs;

pub fn save_music_to_disk(music: Vec<u8>, name: &str) -> Result<(), String> {
    println!("{}", "save_music_to_disk".to_string());
    let path = format!("../file/{}.mp3", name);
    match fs::write(path ,music) {
        Ok(_) => return Ok(()),
        Err(_e) => return Err("could not save file to disk".to_string()),
    };
}
