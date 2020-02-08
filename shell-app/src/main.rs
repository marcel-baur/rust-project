extern crate meff;
use crate::shell::spawn_shell;
use crate::util::Application;
use clap::{App, Arg};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use meff::interface::{start, Peer};

#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate prettytable;

mod shell;
mod util;

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
    let name = matches.value_of("own-name").unwrap_or("Fridolin");
    let port = matches.value_of("port").unwrap_or("34521");
    if matches.is_present("ip-address") {
        // TODO: Join existing p2p network on given ip address
        let addr;
        match matches.value_of("ip-address") {
            Some(ip) => {
                addr = match ip.parse::<SocketAddr>() {
                    Ok(socket_addr) => socket_addr,
                    Err(_) => {
                        //  error!("Could not parse ip address of remote Peer");
                        return;
                    }
                }
            }
            None => {
                //  error!("Could not parse ip-address");
                return;
            }
        }
        let appl = Application { is_playing: Arc::new(Mutex::new(false)) };
        let appl_rc = Arc::new(Mutex::new(appl.clone()));
        let peer = match start(Box::new(appl), name.to_string(), port.to_string(), Some(addr)) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                return;
            } // error!("Could not join network {:?}", e);
        };
        startup(peer, appl_rc);
    } else {
        let appl = Application { is_playing: Arc::new(Mutex::new(false)) };
        let appl_rc = Arc::new(Mutex::new(appl.clone()));
        let peer = match start(Box::new(appl), name.to_string(), port.to_string(), None) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                return;
            } // error!("Could not join network {:?}", e);
        };
        startup(peer, appl_rc);
    }
}

fn startup(peer: Arc<Mutex<Peer>>, model: Arc<Mutex<Application>>) {
    match spawn_shell(peer, model) {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Failed to spawn shell");
        }
    };
}
