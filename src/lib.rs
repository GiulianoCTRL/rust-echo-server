//! Currently unused implementations that are somehow, sometime being worked on.
#![allow(dead_code)]

use std::net::TcpStream;
use std::io::prelude::*;


fn message_to_string(mut stream: TcpStream) -> String {
    let mut buffer = [0u8; 64];
    let mut content = String::new();

    loop {
        match stream.read(&mut buffer) {
            Ok(i) if i == 64 => content.push_str(&String::from_utf8_lossy(&mut buffer[..i])),
            Ok(i) => {
                content.push_str(&String::from_utf8_lossy(&mut buffer[..i]));
                break
            },
            Err(e) => panic!("Could not read from stream: {}", e),
        }
    }
    content
}

struct Message {
    start_line: StartLine,
    headers: Option<Headers>,
    body: Option<String>,
}

#[derive(Debug, PartialEq)]
enum StartLine {
    Request(RequestStartLine),
    Response(ResponseStartLine),
}
#[derive(Debug, PartialEq)]
struct RequestStartLine {
    method: String,
    target: String,
    version: String,
}

#[derive(Debug, PartialEq)]
struct ResponseStartLine {
    version: String,
    status_code: u16,
    status_text: String,
}

impl StartLine {
    fn get_start_line(m: &str) -> String {
        let sl= m.lines().by_ref().take(1).collect();
        sl
    }

    fn new(m: &str) -> StartLine {
        let raw_sl = StartLine::get_start_line(&m);
        let sl: Vec<&str> = raw_sl.split(" ").collect();
        match Method::get_type(sl[0]) {
            Method::RESPONSE => StartLine::Response(
                ResponseStartLine {
                    version: String::from(sl[0]),
                    status_code: sl[1].parse::<u16>().expect("Invalid status code."),
                    status_text: sl[2..].join(" "),
                }
            ),
            _ => StartLine::Request(
                RequestStartLine {
                    method: String::from(sl[0]),
                    target: String::from(sl[1]),
                    version: String::from(sl[2]),
                }
            ),
        }
    }
}



struct Header {
    name: String,
    content: String,
}

struct Headers {
    // Response or request header give additional information about sender/header
    type_headers: Option<Vec<Header>>,
    // General headers apply to whole message
    general_headers: Option<Vec<Headers>>,
    // Entity headers apply to content of body, usually omitted if no content
    entity_headers: Option<Vec<Headers>>,
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
    RESPONSE,
}

impl Method {
    // This function should return Result<Method, HttpError>
    fn get_type(content: &str) -> Method {
        match &content[..3] {
            "GET" => Method::GET,
            "HEA" => Method::HEAD,
            "POS" => Method::POST,
            "PUT" => Method::PUT,
            "DEL" => Method::DELETE,
            "CON" => Method::CONNECT,
            "OPT" => Method::OPTIONS,
            "TRA" => Method::TRACE,
            "PAT" => Method::PATCH,
            "HTT" => Method::RESPONSE,
            _ => panic!("Not a valid HTTP Method/Response."),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    static REQ: &str = "GET / HTTP/1.1\r\nContent-Length: 10\r\n0123456789";
    static RESP: &str = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\nPretty cool!";

    #[test]
    fn test_get_start_line_req() {
        let expected = String::from("GET / HTTP/1.1");
        assert_eq!(
            StartLine::get_start_line(&*REQ),
            expected,
        )
    }

    #[test]
    fn test_get_start_line_resp() {
        let expected = String::from("HTTP/1.1 200 OK");
        assert_eq!(
            StartLine::get_start_line(&*RESP),
            expected,
        )
    }

    #[test]
    fn test_new_req_start_line() {
        let expected = StartLine::Request(RequestStartLine {
            method: String::from("GET"),
            target: String::from("/"),
            version: String::from("HTTP/1.1"),
        });
        let result = StartLine::new(&*REQ);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_resp_start_line() {
        let expected = StartLine::Response(ResponseStartLine {
            version: String::from("HTTP/1.1"),
            status_code: 200u16,
            status_text: String::from("OK"),
        });
        let result = StartLine::new(&*RESP);
        assert_eq!(result, expected);
    }
}