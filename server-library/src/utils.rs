use serde::{Deserialize, Serialize};
use std::time;

/// The sleep duration for the heartbeat thread.
pub const HEARTBEAT_SLEEP_DURATION: time::Duration = time::Duration::from_secs(100);

/// Enum to communicate file instructions to the library.
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum FileInstructions {
    PLAY,
    GET,
    ORDER,
    REMOVE,
}

/// Enum to get the details for the `file_status_changed` function in `AppListener`
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum FileStatus {
    NEW,
    DELETE,
    DOWNLOAD,
}

/// The trait that needs to be implemented for the listener in the application that uses this
/// library. Its functions are used to communicate network events.
pub trait AppListener: Send {
    /// Notify the application that a status response was received.
    /// # Parameters
    /// - `files` the files from the peer that sent the response
    /// - `name` the name of the peer that sent the response
    fn notify_status(&self, files: Vec<String>, name: String);
    /// Notify if a file was changed in the local peer database or was downloaded
    fn local_database_changed(&mut self, name: String, instr: FileStatus);
    /// Notify the application that the player started playing
    /// # Parameters
    /// - `title`: The name of the song has started to play. `None` if streaming.
    fn player_playing(&mut self, title: Option<String>);
    /// Notify the application that the player has stopped (no other song in queue)
    fn player_stopped(&mut self);
}
