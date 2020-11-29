use rust_echo_server::{TcpResponse, TcpServer};
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

// The hell am I doing?
// TODO: Fix... everything!
fn main() {
    let server = TcpServer::new("127.0.0.1", "8080");
    server.handle_streams(handle_connection);
}

fn handle_connection(mut stream: TcpStream) {
    // Figure out a way to read the whole request without it not closing properly.
    let mut buffer = [0; 4096];

    stream.read(&mut buffer).unwrap();
    let mut response = TcpResponse::new(String::from_utf8_lossy(&mut buffer[..]).to_string());
    println!("{}", response.content());
    stream.write(response.content().as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}
