use serde::{Deserialize, Serialize};
use std::time;

pub const THREAD_SLEEP_DURATION: time::Duration = time::Duration::from_millis(5000);

pub const HEARTBEAT_SLEEP_DURATION: time::Duration = time::Duration::from_secs(10);

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum Instructions {
    PLAY,
    GET,
    ORDER,
    REMOVE,
}

/// The trait that needs to be implemented for the listener in the application that uses this
/// library. Its functions are used to communicate network events.
pub trait AppListener: Send {
    fn notify(&self);
    /// Notify the application that a status response was received.
    /// # Parameters
    /// - `files` the files from the peer that sent the response
    /// - `name` the name of the peer that sent the response
    fn notify_status(&self, files: Vec<String>, name: String);
    fn new_file_saved(&mut self, name: String);
}
