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
        network::join_network(name, port, addr);
    } else {
        // TODO: Create new p2p network
        let join_handle = network::startup(name.parse().unwrap(), port.parse().unwrap());
        join_handle.join().expect_err("Could not spawn peer");
    }
}
