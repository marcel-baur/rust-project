use crate::constants;
use crate::network::peer::Peer;
use crate::network::{send_delete_peer_request, send_read_request, send_write_request};
use std::error::Error;
use std::fs;
use std::io::{stdin, ErrorKind};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{io, thread};

pub fn spawn_shell(arc: Arc<Mutex<Peer>>) -> Result<(), Box<dyn Error>> {
    let interaction_in_progress = Arc::new(AtomicBool::new(false));
    let i_clone = interaction_in_progress.clone();
    // Use the peer clone, drop the original alloc of the peer
    let peer = arc.lock().unwrap();
    let ip = *peer.get_ip();
    drop(peer);
    let _handle = thread::Builder::new()
        .name("Interaction".to_string())
        .spawn(move || loop {
            i_clone.store(true, Ordering::SeqCst);
            handle_user_input(ip);
            i_clone.store(false, Ordering::SeqCst);
        })
        .unwrap();

    loop {
        if !interaction_in_progress.load(Ordering::SeqCst) {
            println!("Interaction Possible");
            // show_db_status(&peer_clone_write);
        }
        thread::sleep(constants::THREAD_SLEEP_DURATION);
    }
}

pub fn show_db_status(peer: Peer) {
    //println!("Current state of local database");
    // TODO: Print current keys of db
    println!("Show");
    for k in peer.get_db().get_data() {
        println!("{:?}", k);
    }
}

pub fn handle_user_input(ip: SocketAddr) {
    loop {
        let buffer = &mut String::new();
        stdin().read_line(buffer).unwrap();
        buffer.trim_end();
        let mut buffer_iter = buffer.split_whitespace();
        let instructions: Vec<&str> = buffer_iter.collect();
        match instructions.first() {
            Some(&"h") => {
                show_help_instructions();
            }
            Some(&"help") => {
                show_help_instructions();
            }
            Some(&"push") => {
                if instructions.len() == 3 {
                    //                let mutex = *peer.lock().unwrap();
                    //                push_music_to_database(instructions[1], instructions[2], mutex);
                    push_music_to_database(instructions[1], instructions[2], ip);
                } else {
                    println!(
                        "You need to specify name and filepath. For more information type help.\n"
                    );
                }
            }
            Some(&"get") => {
                if instructions.len() == 2 {
                    send_read_request(ip, instructions[1]);
                } else {
                    println!(
                        "You need to specify name and filepath. For more information type help.\n"
                    );
                }
            }
            Some(&"exit") => {
                println!("You are leaving the network.");
                send_delete_peer_request(ip);
                //TODO: stop steams
            }

            _ => println!("No valid instructions. Try help!\n"),
        }
    }
}

pub fn show_help_instructions() {
    let info = "\nHelp Menu:\n\n\
                Use following instructions: \n\n\
                push [mp3 name] [direction to mp3] - add mp3 to database\n\
                get [mp3 name] - get mp3 file from database\n\
                stream [mp3 name] - get mp3 stream from database\n\
                remove [mp3 name] - deletes mp3 file from database\n\
                exit - exit network and leave program\n\n
                ";
    print!("{}", info);
}

/// Function to check file path to mp3 and saves to db afterwards
/// # Arguments:
///
/// * `name` - String including mp3 name (key in our database)
/// * `file_path` - Path to the mp3 file
/// * `peer` - Peer
///
/// # Returns:
/// Result //@TODO
pub fn push_music_to_database(
    name: &str,
    file_path: &str,
    addr: SocketAddr,
) -> Result<(), io::Error> {
    // get mp3 file
    let path = Path::new(file_path);
    if path.exists() {
        let read_result = fs::read(path);
        match read_result {
            Ok(content) => {
                println!("file eingelesen");
                //@TODO save to database
                //                peer.get_db().add_file(name, content);
                //                peer.store((name.parse().unwrap(), content));
                send_write_request(addr, addr, (name.to_string(), content), false);
                println!("saved to hash map");
                return Ok(());
            }
            Err(err) => {
                println!("Error while parsing file");
                return Err(err);
            }
        }
    } else {
        println!("The file could not be found at this path: {:?}", path);
    }
    return Err(io::Error::new(ErrorKind::NotFound, "File Path not found!"));
}
