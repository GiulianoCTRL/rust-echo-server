//! Currently unused implementations that are somehow, sometime being worked on.
#![allow(dead_code)]

use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug, PartialEq)]
struct Message {
    start_line: StartLine,
    headers: HashMap<String, String>,
    body: String,
}

impl Message {
    // TODO: Should return Result<T>
    fn stream_to_string(mut stream: TcpStream) -> String {
        let mut buffer = [0u8; 64];
        let mut content = String::new();

        loop {
            match stream.read(&mut buffer) {
                Ok(i) if i == 64 => content.push_str(&String::from_utf8_lossy(&buffer[..i])),
                Ok(i) => {
                    content.push_str(&String::from_utf8_lossy(&buffer[..i]));
                    break;
                }
                Err(e) => panic!("Could not read from stream: {}", e),
            }
        }
        content
    }

    fn get_headers_and_body(m: &str) -> (HashMap<String, String>, String) {
        let mut headers: HashMap<String, String> = HashMap::new();
        let mut body = String::from("");
        // Skip start-line
        let mut lines = m.lines();
        lines.next();

        let mut reached = false;
        for line in lines {
            if !reached && line.contains(": ") {
                let header_data: Vec<&str> = line.split(": ").collect();
                headers.insert(header_data[0].to_string(), header_data[1].to_string());
            } else {
                body.push_str(line);
                reached = true;
            }
        }

        (headers, body)
    }

    fn new_from_string(message: &str) -> Message {
        let start_line = StartLine::new(&message);
        let (headers, body) = Message::get_headers_and_body(&message);

        Message {
            start_line,
            headers,
            body,
        }
    }

    fn new_from_stream(stream: TcpStream) -> Message {
        let message = Message::stream_to_string(stream);
        Message::new_from_string(&message)        
    }
}

#[derive(Debug, PartialEq)]
enum StartLine {
    Response {
        version: String,
        status_code: u16,
        status_text: String,
    },
    Request {
        method: String,
        target: String,
        version: String,
    },
}

impl StartLine {
    fn get_start_line(m: &str) -> String {
        let sl = m.lines().by_ref().take(1).collect();
        sl
    }

    fn new(m: &str) -> StartLine {
        let raw_sl = StartLine::get_start_line(&m);
        let sl: Vec<&str> = raw_sl.split(' ').collect();
        match Method::get_type(sl[0]) {
            Method::RESPONSE => StartLine::Response {
                version: String::from(sl[0]),
                status_code: sl[1].parse::<u16>().expect("Invalid status code."),
                status_text: sl[2..].join(" "),
            },
            _ => StartLine::Request {
                method: String::from(sl[0]),
                target: String::from(sl[1]),
                version: String::from(sl[2]),
            },
        }
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
    fn test_req_get_start_line() {
        let expected = String::from("GET / HTTP/1.1");
        assert_eq!(StartLine::get_start_line(&*REQ), expected,)
    }

    #[test]
    fn test_resp_get_start_line() {
        let expected = String::from("HTTP/1.1 200 OK");
        assert_eq!(StartLine::get_start_line(&*RESP), expected,)
    }

    #[test]
    fn test_req_new_start_line() {
        let expected = StartLine::Request {
            method: String::from("GET"),
            target: String::from("/"),
            version: String::from("HTTP/1.1"),
        };
        let result = StartLine::new(&*REQ);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_resp_new_start_line() {
        let expected = StartLine::Response {
            version: String::from("HTTP/1.1"),
            status_code: 200u16,
            status_text: String::from("OK"),
        };
        let result = StartLine::new(&*RESP);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_req_get_headers_and_body() {
        let (headers, body) = Message::get_headers_and_body(&*REQ);
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Content-Length".to_string(), "10".to_string());

        assert_eq!(headers, headers_expected);
        assert_eq!(body, "0123456789".to_string());
    }

    #[test]
    fn test_resp_get_headers_and_body() {
        let (headers, body) = Message::get_headers_and_body(&*RESP);
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Content-Length".to_string(), "12".to_string());

        assert_eq!(headers, headers_expected);
        assert_eq!(body, "Pretty cool!".to_string());
    }

    #[test]
    fn test_req_new_from_string() {
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Content-Length".to_string(), "10".to_string());
        
        let expected = Message {
            start_line: StartLine::Request {
                method: String::from("GET"),
                target: String::from("/"),
                version: String::from("HTTP/1.1"),
            },
            headers: headers_expected,
            body: "0123456789".to_string(),
        };

        let result = Message::new_from_string(&*REQ);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_resp_new_from_string() {
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Content-Length".to_string(), "12".to_string());
        
        let expected = Message {
            start_line: StartLine::Response {
                version: String::from("HTTP/1.1"),
                status_code: 200u16,
                status_text: String::from("OK"),
            },
            headers: headers_expected,
            body: "Pretty cool!".to_string(),
        };

        let result = Message::new_from_string(&*RESP);
        assert_eq!(expected, result);
    }
}
