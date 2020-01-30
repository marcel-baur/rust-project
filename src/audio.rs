use crate::network::peer::Peer;
use crate::network::send_read_request;
use crate::utils::Instructions::PLAY;
use rodio::{Sink};
use std::{fs, thread};
use std::io::BufReader;
use std::string::ToString;
use serde::{Deserialize, Serialize};
use std::sync::Arc;


#[derive(Deserialize, Serialize, Debug)]
pub enum MusicState {
    PLAY, PAUSE, STOP, CONTINUE,
}

pub fn create_sink() -> Result<Sink, String> {
    let device = match rodio::default_output_device() {
        Some(device) => device,
        None => return Err("No output device found".to_string()),
    };
    Ok(Sink::new(&device))
}

/// plays audio when mp3 is in database otherwise sends request to find file
/// # Arguments:
///
/// * `name` - String including mp3 name (key in our database)
///
pub fn play_music(peer: &mut Peer, name: &str, sink: Arc<Sink>) {
    // we play sound when it is in our own database, otherwise we ask for the location
    let sound_data = match peer.get_db().data.get(name) {
        Some(data) => data,
        None => {
            send_read_request(peer.ip_address, name, PLAY);
            return;
        }
    };
    play_music_by_vec(sound_data, sink);
}

pub fn pause_current_playing_music(sink: Arc<Sink>) {
    sink.pause();
}

pub fn stop_current_playing_music(sink: Arc<Sink>) {
    sink.stop();
}

pub fn continue_paused_music(sink: Arc<Sink>) {
    sink.play();
}

pub fn play_music_by_vec(music: &Vec<u8>, sink: Arc<Sink>) -> Result<(), String> {
    match fs::write("file/tmp.mp3", music) {
        Ok(_) => {}
        Err(_e) => return Err("could not save file to disk".to_string()),
    };
    let file = match std::fs::File::open("file/tmp.mp3") {
        Ok(file) => file,
        Err(_e) => return Err("could not read file from disk".to_string()),
    };
    let source = match rodio::Decoder::new(BufReader::new(file)) {
        Ok(decoded_source) => decoded_source,
        Err(_e) => return Err("file could not be decoded. is it mp3?".to_string()),
    };


    let audio_thread = thread::Builder::new()
        .name("AudioThread".to_string())
        .spawn(move || {
            sink.append(source);
            sink.sleep_until_end();
        })
        .unwrap();



    //hier neuen thread mac hen
    //sink.detach();




    //@TODO use sink here to control audio, i.e. pause, stop
    // BufferReader extension trait?
    //rodio::play_raw(&device, source.convert_samples());
    //rodio::play_once(&device, source.convert_samples());
    /**match fs::remove_file("file/tmp.mp3") {
        Ok(_) => return Ok(()),
        Err(_e) => {
            return Err("Could not delete file from disk".to_string());
        } //TODO review},
    };**/
        Ok(())
}
