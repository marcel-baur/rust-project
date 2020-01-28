extern crate clap;
#[macro_use]
extern crate prettytable;

use clap::{App, Arg};
use std::net::SocketAddr;

mod audio;
mod utils;
mod database;
mod network;
mod shell;

fn main() {
    let matches = App::new("MEFF-Music")
        .version("0.1.0")
        .arg(
            Arg::with_name("own-name")
                .short("n")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .takes_value(true)
                .required(false)
                .index(2),
        )
        .arg(
            Arg::with_name("ip-address")
                .short("ip")
                .takes_value(true)
                .required(false)
                .index(3),
        )
        .get_matches();
    let name = matches.value_of("own-name").unwrap();
    let port = matches.value_of("port").unwrap_or("34521");
    if matches.is_present("ip-address") {
        // TODO: Join existing p2p network on given ip address
        let addr;
        match matches.value_of("ip-address") {
            Some(ip) => {
                dbg!(ip);
                addr = match ip.parse::<SocketAddr>() {
                    Ok(socket_addr) => socket_addr,
                    Err(_) => {
                        println!("Could not parse ip address of remote Peer");
                        return;
                    }
                }
            }
            None => {
                println!("Could not parse ip-address");
                return;
            }
        }
        match network::startup(name, port, Some(addr)) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Could not join network");
            }
        };
    } else {
        match network::startup(name, port, None) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Could not join network");
            }
        };
    }
}
