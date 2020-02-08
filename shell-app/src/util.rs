use crate::shell;
use meff::utils::{AppListener, ListenerInstr};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Application {
    pub is_playing: Arc<Mutex<bool>>,
}

impl AppListener for Application {
    fn notify(&self) {
        println!("Hello world");
    }
    fn notify_status(&self, files: Vec<String>, name: String) {
        shell::print_external_files(files, name);
    }
    fn file_status_changed(&mut self, name: String, instr: ListenerInstr) {
        match instr {
            ListenerInstr::NEW => {
                println!("New file {} saved!", name);
            }
            ListenerInstr::DELETE => {
                println!("Deleted file {}!", name);
            }
            ListenerInstr::DOWNLOAD => {
                println!("Download file {} successfully!", name);
            }
        }
    }

    fn player_playing(&mut self, title: Option<String>) {
        if let Some(name) = title {
            println!("{} is playing!", name);
        }
        *self.is_playing.lock().unwrap() = true;
    }

    fn player_stopped(&mut self) {
        *self.is_playing.lock().unwrap() = false;
    }
}
