use crate::network::notification::{Content, Notification};
use std::net::{SocketAddr, TcpStream};
use std::time::SystemTime;

/// Sends a request to the other peers to check if they have the wanted file
pub fn read_file_exist(target: SocketAddr, from: SocketAddr, name: &str, id: SystemTime) {
    let stream = TcpStream::connect(target).unwrap();

    let not = Notification {
        content: Content::ExistFile {
            song_name: name.to_string(),
            id,
        },
        from,
    };

    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

/// Sends a response (to ExistFile Request) to let one peer know to have a requested file
pub fn send_exist_response(target: SocketAddr, from: SocketAddr, name: &str, id: SystemTime) {
    let stream = match TcpStream::connect(target) {
        Ok(s) => s,
        Err(_e) => {
            eprintln!("Failed to connect to {:?}", target);
            return;
        }
    };
    let not = Notification {
        content: Content::ExistFileResponse {
            song_name: name.to_string(),
            id,
        },
        from,
    };
    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

/// Sends a request (as a response of ExistFileResponse Request) to get a certain file
pub fn send_file_request(target: SocketAddr, from: SocketAddr, name: &str) {
    let stream = match TcpStream::connect(target) {
        Ok(s) => s,
        Err(_e) => {
            eprintln!("Failed to connect to {:?}", target);
            return;
        }
    };
    let not = Notification {
        content: Content::GetFile {
            key: name.to_string(),
        },
        from,
    };
    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

/// Sends a response to a GetFile Request containing the music data
pub fn send_get_file_reponse(target: SocketAddr, from: SocketAddr, key: &str, value: Vec<u8>) {
    let stream = match TcpStream::connect(target) {
        Ok(s) => s,
        Err(_e) => {
            eprintln!("Failed to connect to {:?}", target);
            return;
        }
    };
    let not = Notification {
        content: Content::GetFileResponse {
            key: key.to_string(),
            value,
        },
        from,
    };
    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}

pub fn song_order_request(target: SocketAddr, from: SocketAddr, song_name: String) {
    let stream = match TcpStream::connect(target) {
        Ok(s) => s,
        Err(_e) => {
            eprintln!("Failed to connect to {:?}", target);
            return;
        }
    };
    let not = Notification {
        content: Content::OrderSongRequest {
            song_name,
        },
        from,
    };
    match serde_json::to_writer(&stream, &not) {
        Ok(ser) => ser,
        Err(_e) => {
            println!("Failed to serialize SendRequest {:?}", &not);
        }
    };
}
