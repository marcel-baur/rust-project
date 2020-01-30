use crate::network::peer::Peer;
use crate::network::send_read_request;
use crate::utils::Instructions::PLAY;
use rodio::{Sink};
use std::{fs, thread};
use std::io::{BufReader, Cursor};
use std::string::ToString;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::io::BufRead;
use std::io::Read;



#[derive(Deserialize, Serialize, Debug)]
pub enum MusicState {
    PLAY, PAUSE, STOP, CONTINUE,
}

pub struct MusicPlayer {
    sink: Sink,
    is_playing: bool,
}

pub fn create_sink() -> Result<MusicPlayer, String> {
    let device = match rodio::default_output_device() {
        Some(device) => device,
        None => return Err("No output device found".to_string()),
    };
    Ok(MusicPlayer {
        sink: Sink::new(&device),
        is_playing: false,
    })
}

/// plays audio when mp3 is in database otherwise sends request to find file
/// # Arguments:
///
/// * `name` - String including mp3 name (key in our database)
///
pub fn play_music(peer: &mut Peer, name: &str, sink: &mut MusicPlayer) {
    // we play sound when it is in our own database, otherwise we ask for the location
    let sound_data = match peer.get_db().data.get(name) {
        Some(data) => data,
        None => {
            send_read_request(peer.ip_address, name, PLAY);
            return;
        }
    };
    play_music_by_vec(sound_data.clone(), sink);
}

pub fn pause_current_playing_music(sink: &mut MusicPlayer) {
    sink.sink.pause();
}

pub fn stop_current_playing_music(sink: &mut MusicPlayer) {
    sink.sink.stop();
    sink.is_playing = false;
}

pub fn continue_paused_music(sink: &mut MusicPlayer) {
    sink.sink.play();
}

pub fn play_music_by_vec(music: Vec<u8>, sink: &mut MusicPlayer) -> Result<(), String> {
    let music_a = Cursor::new(music);
    let file = BufReader::new(music_a);
    let source = match rodio::Decoder::new(file) {
        Ok(decoded_source) => decoded_source,
        Err(_e) => return Err("file could not be decoded. is it mp3?".to_string()),
    };
    if sink.is_playing {
        sink.sink.append(source);
    } else {
        sink.is_playing = true;
        sink.sink = create_sink().unwrap().sink;
        sink.sink.append(source);
    }
    Ok(())
}