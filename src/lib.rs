use std::net::{TcpListener, TcpStream};
use std::thread;
pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub fn new(ip: &str, port: &str) -> TcpServer {
        let listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();
        TcpServer { listener }
    }

    pub fn handle_streams(&self, conn_handler: fn(TcpStream)) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            thread::spawn(move || {
                conn_handler(stream);
            });
        }
    }
}

pub struct TcpResponse {
    content: String,
}

impl TcpResponse {
    pub fn new(content: String) -> TcpResponse {
        let headers = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n", content.len());
        TcpResponse {
            content: format!("{}{}", headers, content),
        }
    }

    pub fn content(&mut self) -> &str {
        &self.content
    }
}
