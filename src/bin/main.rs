use rust_echo_server::TcpServer;
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

// The hell am I doing?
// TODO: Fix... everything!
fn main() {
    let server = TcpServer::new("127.0.0.1", "8080");
    server.handle_streams(handler);
}


// Rough, unoptimised working implementation to test how to correct behaviour
fn handler(mut stream: TcpStream) {
    let mut buffer = [0u8; 64];
    let mut content = String::new();
    let mut bytes_read = stream.read(&mut buffer).unwrap();
    content.push_str(&String::from_utf8_lossy(&mut buffer[..bytes_read]));
    while bytes_read == 64 {
        bytes_read = stream.read(&mut buffer).unwrap();
        content.push_str(&String::from_utf8_lossy(&mut buffer[..bytes_read]));
    }
    if content.contains("Expect: 100-continue") {
        stream.write("HTTP/1.1 100 Continue\r\n\r\n".as_bytes()).unwrap();
        stream.flush().unwrap();
        let mut bytes_read = stream.read(&mut buffer).unwrap();
        content.push_str(&String::from_utf8_lossy(&mut buffer[..bytes_read]));
        while bytes_read == 64 {
            bytes_read = stream.read(&mut buffer).unwrap();
            content.push_str(&String::from_utf8_lossy(&mut buffer[..bytes_read]));
        }
    }


    println!("{}", content);
    stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}
