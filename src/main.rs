extern crate clap;

use clap::{App, Arg};

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
            Some(ip) => addr = ip,
            None => {
                println!("Could not parse ip-address");
                return;
            }
        }
        join_network(name, addr);
    } else {
        // TODO: Create new p2p network
        create_network(name);
    }
}

fn create_network(own_name: &str) {
    // TODO
    println!("Create network with {} as own name", own_name);
}

fn join_network(own_name: &str, ip_address: &str) {
    // TODO
    println!("Join {} as {}", ip_address, own_name);
}
