use crate::shell;
use meff::utils::AppListener;

pub struct Application {}

impl AppListener for Application {
    fn notify(&self) {
        println!("Hello world");
    }
    fn notify_status(&self, files: Vec<String>, name: String) {
        shell::print_external_files(files, name);
    }
}
