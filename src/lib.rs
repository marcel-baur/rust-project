extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate log;
extern crate log4rs;
use clap::{App, Arg};
use std::net::SocketAddr;

pub mod audio;
pub mod database;
pub mod network;
mod shell;
pub mod utils;
