use crate::interface::Peer;
use crate::network::send_read_request;
use crate::utils::FileInstructions::PLAY;
use rodio::Sink;
use std::fs;
use std::io::{BufReader, Cursor};
use std::string::ToString;

pub struct MusicPlayer {
    sink: Sink,
    is_playing: bool,
    current_song_name: Option<String>,
}

pub fn create_sink() -> Result<MusicPlayer, String> {
    let device = match rodio::default_output_device() {
        Some(device) => device,
        None => return Err("No output device found".to_string()),
    };
    Ok(MusicPlayer {
        sink: Sink::new(&device),
        is_playing: false,
        current_song_name: None,
    })
}

pub fn save_music_to_disk(music: Vec<u8>, name: &str) -> Result<(), String> {
    println!("{}", "save_music_to_disk".to_string());
    let path = format!("../file/{}.mp3", name);
    match fs::write(path, music) {
        Ok(_) => Ok(()),
        Err(_e) => Err("could not save file to disk".to_string()),
    }
}

/// plays audio when mp3 is in database otherwise sends request to find file
/// # Arguments:
///
/// * `name` - String including mp3 name (key in our database)
///
pub fn play_music(
    peer: &mut Peer,
    name: &Option<String>,
    sink: &mut MusicPlayer,
) -> Result<(), String> {
    // we play sound when it is in our own database, otherwise we ask for the location
    let title;
    if let Some(song_name) = name {
        title = song_name.to_string();
    } else if let Some(song_name) = &sink.current_song_name {
        title = song_name.to_string();
    } else {
        return Err("No song name given and no current playing song!".to_string());
    }

    let sound_data = match peer.get_db().data.get(&title) {
        Some(data) => data,
        None => {
            send_read_request(peer, title.as_ref(), PLAY);
            return Err("File not in local Database. Search for File".to_string());
        }
    };

    play_music_by_vec(sound_data.clone(), sink, title)
}

pub fn pause_current_playing_music(sink: &mut MusicPlayer) -> Result<(), String> {
    sink.sink.pause();
    Ok(())
}

pub fn stop_current_playing_music(sink: &mut MusicPlayer) -> Result<(), String> {
    sink.sink.stop();
    sink.is_playing = false;
    sink.current_song_name = None;
    Ok(())
}

pub fn continue_paused_music(sink: &mut MusicPlayer) -> Result<(), String> {
    sink.sink.play();
    Ok(())
}

pub fn play_music_by_vec(
    music: Vec<u8>,
    sink: &mut MusicPlayer,
    name: String,
) -> Result<(), String> {
    sink.current_song_name = Some(name);
    let music_a = Cursor::new(music);
    let file = BufReader::new(music_a);
    let source = match rodio::Decoder::new(file) {
        Ok(decoded_source) => decoded_source,
        Err(_e) => return Err("file could not be decoded. is it mp3?".to_string()),
    };
    sink.sink.play();
    if sink.is_playing {
        sink.sink.append(source);
    } else {
        sink.is_playing = true;
        sink.sink = create_sink().unwrap().sink;
        sink.sink.append(source);
    }
    Ok(())
}
