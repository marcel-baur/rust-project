use std::string::ToString;
use std::fs;

pub fn save_music_to_disk(music: Vec<u8>, name: &str) -> Result<(), String> {
    println!("{}", "save_music_to_disk".to_string());
    let path = format!("../file/{}.mp3", name);
    match fs::write(path ,music) {
        Ok(_) =>  Ok(()),
        Err(_e) => Err("could not save file to disk".to_string()),
    }
}
