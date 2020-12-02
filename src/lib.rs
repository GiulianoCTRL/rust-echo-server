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

trait HttpServer {
    fn get_stream(&self) -> TcpStream;

    fn process_post(&self) ;
}
