use crate::constants;
use crate::network::{handle_user_input, Peer};
use std::error::Error;
use std::io::stdin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn spawn_shell(arc: Arc<Mutex<Peer>>) -> Result<(), Box<dyn Error>> {
    let interaction_in_progress = Arc::new(AtomicBool::new(false));
    let i_clone = interaction_in_progress.clone();
    let peer = arc.lock().unwrap();
    // Use the peer clone, drop the original alloc of the peer
    let peer_clone = peer.clone();
    let peer_clone_write = peer.clone();
    drop(peer);
    let _handle = thread::Builder::new()
        .name("Interaction".to_string())
        .spawn(move || loop {
            let buffer = &mut String::new();
            stdin().read_line(buffer).unwrap();
            if let "m" = buffer.trim_end() {
                i_clone.store(true, Ordering::SeqCst);
                handle_user_input(buffer, peer_clone.clone()); // TODO
                i_clone.store(false, Ordering::SeqCst);
            }
        })
        .unwrap();

    loop {
        if !interaction_in_progress.load(Ordering::SeqCst) {
            println!("Action dispatched.");

            show_db_status(&peer_clone_write);
        }
        thread::sleep(constants::THREAD_SLEEP_DURATION);
    }
}

pub fn show_db_status(peer: &Peer) {
    println!("Current state of local database");
    // TODO: Print current keys of db
    for k in peer.get_db().get_data() {
        println!("{:?}", k);
    }
}
