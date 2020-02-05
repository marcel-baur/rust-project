use meff::utils::AppListener;
use meff::network;
use std::net::SocketAddr;

//Music entertainment for friends application model
#[derive(Clone)]
pub struct MEFFM {
    pub songs: Vec<String>
}

impl AppListener for MEFFM {
    fn notify(&self) {
        println!("Hello world");
    }

    fn notify_status(&self, files: Vec<String>, name: String) {
        unimplemented!()
    }

    fn new_file_saved(&mut self, name: String) {

    }
}

impl MEFFM {
    pub fn new() -> MEFFM {
        let mut songs = Vec::new();
        MEFFM { songs }
    }

    pub fn start(&self, name: String, port: String, ip: Option<SocketAddr>) {
        match network::startup(&name, &port, None, Box::new(self.clone())) {
            Ok(p) => p,
            Err(_e) => {
                return;
            } // error!("Could not join network {:?}", e);
        };
    }
}

