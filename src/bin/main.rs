use rust_echo_server::{TcpResponse, TcpServer};
use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};

// The hell am I doing?
// TODO: Fix... everything!
fn main() {
    let server = TcpServer::new("127.0.0.1", "8080");
    server.handle_streams(handle_stream);
}

fn handle_connection(mut stream: TcpStream) {
    // Figure out a way to read the whole request without it not closing properly.
    let mut buffer = [0; 4096];

    stream.read(&mut buffer).unwrap();
    let mut response = TcpResponse::new(String::from_utf8_lossy(&mut buffer[..]).to_string());
    println!("{}", String::from_utf8_lossy(&mut buffer[..]));
    stream.write(response.content().as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}

use std::io::BufReader;
fn handle_stream(mut stream: TcpStream) {
    // Seems like we do not read the content until the client stream closes    
    stream.write("HTTP/1.1 200 OK\r\nAccept: */*\r\n".as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut reader = BufReader::new(stream);
    let mut s = String::new();
    loop {
        match reader.read_line(&mut s) {
            Ok(rb) if rb == 0 => break,
            // Ok(rb) if rb <= 2 => break,  // The CRLF is causing the reader to stop reading to early?
            Ok(_) => println!("{}", s),
            Err(e) => {
                println!("Error receiving request: {}", e);
                break
            },
        }
    }

    println!("{}", s);
}
