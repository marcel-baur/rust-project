use crate::network::notification::{Content, Notification, tcp_request_with_notification};
use std::net::{SocketAddr, TcpStream};
use std::time::SystemTime;
use crate::utils::Instructions;

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
pub fn send_file_request(target: SocketAddr, from: SocketAddr, name: &str, instr: Instructions) {
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
    instr: Instructions,
) {

    let not = Notification {
        content: Content::GetFileResponse {
            instr,
            key: key.to_string(),
            value,
        },
        from,
    };

    tcp_request_with_notification(target, not);
}

pub fn song_order_request(target: SocketAddr, from: SocketAddr, song_name: String) {
    let not = Notification {
        content: Content::OrderSongRequest { song_name },
        from,
    };

    tcp_request_with_notification(target, not);
}

/// Sends a request to delete redundant file
pub fn delete_redundant_song_request(target: SocketAddr, from: SocketAddr, song_name: String) {
    let not = Notification {
        content: Content::DeleteFileRequest { song_name },
        from,
    };

    tcp_request_with_notification(target, not);
}
