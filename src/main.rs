use std::{net::TcpListener, time::Duration};

// Configuration, will put in a file config soon.
pub const BIND_IP: &str = "127.0.0.1";
pub const PORT: &str = "25565";
pub const STREAM_READ_TIMEOUT: Duration = Duration::from_millis(100); // Probably too long (idk).
pub const STREAM_WRITE_TIMEOUT: Duration = Duration::from_millis(100); // Probably too long (idk).

// Not yet used constants.
pub const _CONNECTION_STREAM_THREAD_POOL_SIZE: i32 = 4;

pub mod byte_helpers;
pub mod connection_handler;
pub mod log;
pub mod macros;
pub mod protocol;

fn main() {
    let tcp_listener =
        TcpListener::bind(format!("{BIND_IP}:{PORT}")).expect("Failed to bind TcpListener.");

    // TODO: Use a thread pool.
    for stream in tcp_listener.incoming() {
        println!("[NEW CONNECTION]");
        connection_handler::handle_connection(stream.unwrap());
    }
}
