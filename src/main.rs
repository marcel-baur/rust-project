extern crate clap;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate log;
extern crate log4rs;
use clap::{App, Arg};
use std::net::SocketAddr;

mod audio;
mod database;
mod network;
mod shell;
mod utils;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    info!("Starting...");
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
                info!("{}", ip);
                addr = match ip.parse::<SocketAddr>() {
                    Ok(socket_addr) => socket_addr,
                    Err(_) => {
                        error!("Could not parse ip address of remote Peer");
                        return;
                    }
                }
            }
            None => {
                error!("Could not parse ip-address");
                return;
            }
        }
        if let Err(e) = network::startup(name, port, Some(addr)) {
            error!("Could not join network {:?}", e);
        }
    } else {
        if let Err(e) = network::startup(name, port, None) {
            error!("Could not join network {:?}", e);
        }
        //        match network::startup(name, port, None) {
        //            Ok(_) => {}
        //            Err(e) => {
        //                error!("Could not join network {:?}", e);
        //            }
        //        };
    }
}
