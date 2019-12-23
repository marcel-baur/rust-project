extern crate clap;

use clap::{App, Arg};
use std::net::SocketAddr;

mod constants;
mod database;
mod network;
mod shell;

fn main() {
    let matches = App::new("music_p2p")
        .version("0.1.0")
        .arg(
            Arg::with_name("own-name")
                .short("n")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("ip-address")
                .short("ip")
                .takes_value(true)
                .required(false)
                .index(2),
        )
        .get_matches();
    let name;
    match matches.value_of("own-name") {
        Some(n) => name = n,
        None => {
            println!("No name given!");
            return;
        }
    }
    if matches.is_present("ip-address") {
        // TODO: Join existing p2p network on given ip address
        let addr;
        match matches.value_of("ip-address") {
            Some(ip) => {
                addr = match ip.parse::<SocketAddr>() {
                    Ok(socket_addr) => socket_addr,
                    Err(e) => {
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
        network::join_network(name, addr);
    } else {
        // TODO: Create new p2p network
        let peer = match network::create_network(name) {
            Ok(p) => p,
            Err(str) => {
                println!("{}", str);
                return;
            }
        };
    }
}
