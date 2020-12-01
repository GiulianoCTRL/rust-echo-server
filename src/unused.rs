//! Currently unused implementations that are somehow, sometime being worked on.
#![allow(dead_code)]


// Will be used for testing later
const EX_REQ: String = "GET / HTTP/1.1\r\nContent-Length: 10\r\n0123456789";
const EX_RESP: String = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\nPretty cool!";

struct Request {
    start_line: RequestStartLine,
    headers: Option<Headers>,
    body: Option<Body>,
}

struct Response {
    start_line: ResponseStartLine,
    headers: Option<Headers>,
    body: Option<Body>,
}

struct RequestStartLine {
    method: String,
    target: String,
    version: String,
}

struct ResponseStartLine {
    version: String,
    status_code: u16,
    status_text: String,
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

/// Body of request or response. Responses like 201 or 204 usually do not need a body
/// GET, HEAD, DELETE, OPTIONS requests usually do not need/have bodies
struct Body(String);

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
