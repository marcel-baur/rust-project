use serde::{Deserialize, Serialize};
use std::time;
use crate::interface::ListenerInstr;

pub const THREAD_SLEEP_DURATION: time::Duration = time::Duration::from_millis(5000);

pub const HEARTBEAT_SLEEP_DURATION: time::Duration = time::Duration::from_secs(100);


/// The trait that needs to be implemented for the listener in the application that uses this
/// library. Its functions are used to communicate network events.
pub trait AppListener: Send {
    fn notify(&self);
    /// Notify the application that a status response was received.
    /// # Parameters
    /// - `files` the files from the peer that sent the response
    /// - `name` the name of the peer that sent the response
    fn notify_status(&self, files: Vec<String>, name: String);
    fn file_status_changed(&mut self, name: String, instr: ListenerInstr);
    /// Notify the application that the player started playing
    fn player_playing(&mut self, title: Option<String>);
    /// Notify the application that the player has stopped (no other song in queue)
    fn player_stopped(&mut self);
}
