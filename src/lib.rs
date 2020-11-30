use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fmt;
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

    fn analyse_get(&self) {

    }
}

/// Existing HTTP Methods
/// source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[allow(non_camel_case_types)]
enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
// Create proper error type
#[derive(Debug, Clone)]
struct HttpError;

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Method {
    // This function should return Result<Method, HttpError>
    fn analyse_method(content: &str) -> Method {
        match &content[..4] {
            "GET" => Method::GET,
            "HEA" => Method::HEAD,
            "POS" => Method::POST,
            "PUT" => Method::PUT,
            "DEL" => Method::DELETE,
            "CON" => Method::CONNECT,
            "OPT" => Method::OPTIONS,
            "TRA" => Method::TRACE,
            "PAT" => Method::PATCH,
            _ => panic!("Invalid method type!")
        }
    }
}

trait HttpServer {
    fn get_stream(&self) -> TcpStream;

    fn process_post(&self) ;
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
