use meff::utils::AppListener;

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

    fn start(&self, name: String, port: String, ip: Option<String>) {

    }
}

