use prettytable::format;
extern crate colored;
use crate::network::peer::Peer;
use crate::network::{
    send_delete_peer_request, send_play_request, send_read_request, send_status_request,
    send_write_request,
};
use crate::utils;
use crate::utils::Instructions::{GET, REMOVE};
use colored::*;
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
    let arc_clone = arc.clone();
    let arc_clone2 = arc.clone();
    // Use the peer clone, drop the original alloc of the peer
    let peer = arc.lock().unwrap();
    drop(peer);
    let _handle = thread::Builder::new()
        .name("Interaction".to_string())
        .spawn(move || loop {
            let peer = arc_clone.lock().unwrap();
            drop(peer);
            i_clone.store(true, Ordering::SeqCst);
            handle_user_input(&arc_clone2);
            i_clone.store(false, Ordering::SeqCst);
        })
        .unwrap();

    loop {
        let peer = arc.lock().unwrap();
        let _peer_clone = peer.clone();
        drop(peer);
        thread::sleep(utils::THREAD_SLEEP_DURATION);
    }
}

pub fn handle_user_input(arc: &Arc<Mutex<Peer>>) {
    loop {
        let peer = arc.lock().unwrap();
        let mut peer_clone = peer.clone();
        drop(peer);
        let buffer = &mut String::new();
        stdin().read_line(buffer).unwrap();
        let _ = buffer.trim_end();
        let buffer_iter = buffer.split_whitespace();
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
                    match push_music_to_database(
                        instructions[1],
                        instructions[2],
                        peer_clone.ip_address,
                        &mut peer_clone,
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to push {} to database", instructions[1]);
                            error!(
                                "Could not push {:?} to the database, error code {:?}",
                                instructions, e
                            );
                        }
                    };
                } else {
                    println!(
                        "You need to specify name and filepath. For more information type help.\n"
                    );
                }
            }
            Some(&"get") => {
                if instructions.len() == 2 {
                    send_read_request(peer_clone.ip_address, instructions[1], GET);
                } else {
                    println!(
                        "You need to specify name and filepath. For more information type help.\n"
                    );
                }
            }
            Some(&"exit") => {
                println!("You are leaving the network.");
                send_delete_peer_request(peer_clone.ip_address);
                //TODO: stop steams
            }
            Some(&"status") => {
                print_peer_status(&arc);
                print_local_db_status(&arc);
                print_existing_files(&arc);
            }
            Some(&"play") => {
                if instructions.len() == 2 {
                    send_play_request(instructions[1], peer_clone.ip_address);
                } else {
                    println!("File name is missing. For more information type help.\n");
                }
            }
            Some(&"remove") => {
                if instructions.len() == 2 {
                    send_read_request(peer_clone.ip_address, instructions[1], REMOVE);
                } else {
                    println!(
                        "You need to specify name of mp3 file. For more information type help.\n"
                    );
                }
            }
            Some(&"stream") => {
                if instructions.len() == 2 {
                    println!("Not yet implemented.\n");
                } else {
                    println!(
                        "You need to specify name of mp3 file. For more information type help.\n"
                    );
                }
            }

            _ => println!("No valid instructions. Try help!\n"),
        }
    }
}

pub fn show_help_instructions() {
    let info = "\nHelp Menu:\n\n\
                Use following instructions: \n\n\
                status - show current state of peer\n\
                push [mp3 name] [direction to mp3] - add mp3 to database\n\
                get [mp3 name] - get mp3 file from database\n\
                stream [mp3 name] - get mp3 stream from database\n\
                remove [mp3 name] - deletes mp3 file from database\n\
                play [mp3 name] - plays the audio of mp3 file\n\
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
    peer: &mut Peer,
) -> Result<(), io::Error> {
    // get mp3 file
    let path = Path::new(file_path);
    if path.exists() {
        let read_result = fs::read(path);
        match read_result {
            Ok(content) => {
                println!("Pushing... This can take a while");
                //@TODO save to database
                //                peer.get_db().add_file(name, content);
                //                peer.store((name.parse().unwrap(), content));
                send_write_request(addr, addr, (name.to_string(), content), false, peer);
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
    Err(io::Error::new(ErrorKind::NotFound, "File Path not found!"))
}

fn print_peer_status(arc: &Arc<Mutex<Peer>>) {
    let peer = arc.lock().unwrap();
    let peer_clone = peer.clone();
    drop(peer);
    let nwt = peer_clone.network_table;
    let mut other_peers = table!(["Name".italic().yellow(), "SocketAddr".italic().yellow()]);

    for (name, addr) in nwt {
        other_peers.add_row(row![name, addr.to_string()]);
    }
    other_peers.set_format(*format::consts::FORMAT_BORDERS_ONLY);
    println!(
        "\n\n{}\n{}",
        "Current members in the network"
            .to_string()
            .black()
            .on_white(),
        other_peers
    );
}

/// Print the current status of the local database
/// # Arguments:
/// * `peer` - the local `Peer`
fn print_local_db_status(arc: &Arc<Mutex<Peer>>) {
    let peer = arc.lock().unwrap();
    let peer_clone = peer.clone();
    drop(peer);
    let db = peer_clone.get_db().get_data();
    let mut local_data = table!(["Key".italic().green(), "File Info".italic().green()]);
    for (k, v) in db {
        local_data.add_row(row![k, v.len()]);
    }
    local_data.set_format(*format::consts::FORMAT_BORDERS_ONLY);
    print!(
        "\n\n{}\n{}",
        "Current state of local database".to_string(),
        local_data
    );
}

/// Print the name of all existing files in the database
/// # Arguments
/// * `peer` - the local `Peer`
fn print_existing_files(arc: &Arc<Mutex<Peer>>) {
    let peer = arc.lock().unwrap();
    let peer_clone = peer.clone();
    let mut peer_clone2 = peer.clone();
    drop(peer);
    for v in peer_clone.network_table.values() {
        if *v == *peer_clone.get_ip() {
            continue;
        }
        send_status_request(*v, *peer_clone.get_ip(), &mut peer_clone2);
    }
}

/// Print the name of all files from another peer
/// # Arguments
/// * `files` - `Vec<String>` of filenames from another peer
/// * `peer_name` - the name of the peer that holds the files
pub fn print_external_files(files: Vec<String>, peer_name: String) {
    let mut table = table!(["Key".italic().green()]);
    for k in files {
        table.add_row(row![k]);
    }
    table.set_format(*format::consts::FORMAT_BORDERS_ONLY);
    let text = format!(
        "{} {}",
        "Files stored in peer ".to_string().black().on_white(),
        peer_name
    );
    println!("\n\n{}\n{}", text, table);
}
