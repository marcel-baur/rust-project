use meff::utils::AppListener;

pub struct Application {}

impl AppListener for Application {
    fn notify(&self) {
        println!("Hello world");
    }
}
