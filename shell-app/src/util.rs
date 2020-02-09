use crate::shell;
use meff::utils::{AppListener, FileStatus};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Application {
    pub is_playing: Arc<Mutex<bool>>,
}

impl AppListener for Application {
    fn notify_status(&self, files: Vec<String>, name: String) {
        shell::print_external_files(files, name);
    }
    fn local_database_changed(&mut self, name: String, instr: FileStatus) {
        match instr {
            FileStatus::NEW => {
                println!("New file {} saved!", name);
            }
            FileStatus::DELETE => {
                println!("Deleted file {}!", name);
            }
            FileStatus::DOWNLOAD => {
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
