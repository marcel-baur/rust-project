use crate::interface::Notification;
use crate::network::notification::{tcp_request_with_notification, Content};
use crate::utils::FileInstructions;
use std::net::SocketAddr;
use std::time::SystemTime;

/// Sends a request to the other peers to check if they have the wanted file
pub fn read_file_exist(target: SocketAddr, from: SocketAddr, name: &str, id: SystemTime) {
    let not = Notification {
        content: Content::ExistFile {
            song_name: name.to_string(),
            id,
        },
        from,
    };

    tcp_request_with_notification(target, not);
}

/// Sends a response (to ExistFile Request) to let one peer know to have a requested file
pub fn send_exist_response(target: SocketAddr, from: SocketAddr, name: &str, id: SystemTime) {
    let not = Notification {
        content: Content::ExistFileResponse {
            song_name: name.to_string(),
            id,
        },
        from,
    };

    tcp_request_with_notification(target, not);
}

/// Sends a request (as a response of ExistFileResponse Request) to get a certain file
pub fn send_file_request(target: SocketAddr, from: SocketAddr, name: &str, instr: FileInstructions) {
    let not = Notification {
        content: Content::GetFile {
            instr,
            key: name.to_string(),
        },
        from,
    };

    tcp_request_with_notification(target, not);
}

/// Sends a response to a GetFile Request containing the music data
pub fn send_get_file_reponse(
    target: SocketAddr,
    from: SocketAddr,
    key: &str,
    value: Vec<u8>,
    instr: FileInstructions,
) {
    let not = Notification {
        content: Content::GetFileResponse {
            instr,
            key: key.to_string(),
            value,
        },
        from,
    };

    if let Err(e) = thread::Builder::new()
        .name("send_get_file_reponse_thread".to_string())
        .spawn(move || {
            tcp_request_with_notification(target, not);
        }) {
        error!("could not spwan request_thread");
    }
}

pub fn song_order_request(target: SocketAddr, from: SocketAddr, song_name: String) {
    let not = Notification {
        content: Content::OrderSongRequest { song_name },
        from,
    };

    tcp_request_with_notification(target, not);
}

/// Sends a request to delete redundant file
pub fn delete_redundant_song_request(target: SocketAddr, from: SocketAddr, song_name: &str) {
    let not = Notification {
        content: Content::DeleteFileRequest {
            song_name: song_name.to_string(),
        },
        from,
    };

    tcp_request_with_notification(target, not);
}
