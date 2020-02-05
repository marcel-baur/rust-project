use meff::utils::AppListener;

pub struct Application {}

impl AppListener for Application {
    fn notify(&self) {
        println!("Hello world");
    }

    fn notify_status(&self, files: Vec<String>, name: String) {
        unimplemented!()
    }

    fn new_file_saved(&self, name: String) {
        println!("New file {} saved!", name);
    }
}
