extern crate clap;
#[macro_use]
extern crate log;
extern crate log4rs;

/// # MEFF peer to peer network library for music storage
/// Usage:
///
/// The module `interface` is the interface between your application.
///
/// In order to communicate with your application you need to pass a listener object that implements
/// the `AppListener` trait from `utils`. The library will communicate network events through that
/// object using the listener pattern.
///
pub(self) mod audio;
pub(self) mod database;
pub mod interface;
pub(self) mod network;
pub mod utils;
