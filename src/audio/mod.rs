use crate::network::peer::Peer;
use crate::network::send_read_request;
use rodio::Source;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::string::ToString;
use std::time::SystemTime;

/// plays audio when mp3 is in database otherwise sends request to find file
/// # Arguments:
///
/// * `name` - String including mp3 name (key in our database)
///
pub fn play_music(peer: &mut Peer, name: &str) -> Result<(), String> {
    // we play sound when it is in our own database, otherwise we ask for the location
    let sound_data = match peer.get_db().data.get(name) {
        Some(data) => data,
        None => {
            peer.set_waiting_to_play(true);
            send_read_request(peer.ip_address, name);
            return Err("File not in Database, send_request sent".to_string());
        }
    };
    play_music_by_vec(sound_data)
}

pub fn play_music_by_vec(music: &Vec<u8>) -> Result<(), String> {
    let device = match rodio::default_output_device() {
        Some(device) => device,
        None => return Err("No output device found".to_string()),
    };
    match fs::write("file/tmp.mp3", music) {
        _ => {}
        Err(e) => return Err("could not save file to disk".to_string()),
    };
    let file = match std::fs::File::open("file/tmp.mp3") {
        Ok(file) => file,
        Err(e) => return Err("could not read file from disk".to_string()),
    };
    let source = match rodio::Decoder::new(BufReader::new(file)) {
        Ok(decodedSource) => decodedSource,
        Err(e) => return Err("file could not be decoded. is it mp3?".to_string()),
    };
    //@TODO use sink here to control audio, i.e. pause, stop
    rodio::play_raw(&device, source.convert_samples());
    fs::remove_file("file/tmp.mp3");
    Ok(())
}
