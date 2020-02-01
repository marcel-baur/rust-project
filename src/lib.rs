extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate log;
extern crate log4rs;
use clap::{App, Arg};
use std::net::SocketAddr;


/// # MEFF peer to peer network library for music storage
/// ## Usage
/// Use the startup method in `network` to create or join a network
///
/// In order to communicate with your application you need to pass a listener object that implements
/// the `AppListener` trait from `utils`. The library will communicate network events through that
/// object using the listener pattern.
///
pub mod audio;
pub mod database;
pub mod network;
mod shell;
pub mod utils;


