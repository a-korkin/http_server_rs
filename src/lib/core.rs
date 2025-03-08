use crate::request::Request;
use crate::response::Response;
use std::fmt::Display;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    UNKNOWN,
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            _ => Self::UNKNOWN,
        }
    }
}

#[derive(Debug)]
pub enum HttpStatus {
    Status200,
    Status404,
    Status500,
    Undefined,
}

impl From<&str> for HttpStatus {
    fn from(value: &str) -> Self {
        match value {
            "200 OK" => Self::Status200,
            "404 Not Found" => Self::Status404,
            "500 Internal Server Error" => Self::Status500,
            _ => Self::Undefined,
        }
    }
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Status200 => write!(f, "200 OK"),
            Self::Status404 => write!(f, "404 Not Found"),
            Self::Status500 => write!(f, "500 Internal Server Error"),
            _ => write!(f, "Status Unknown"),
        }
    }
}

pub fn run(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_request(stream);
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut buf = [0; 128];
    let mut stop = false;
    let mut request_array: Vec<u8> = vec![0];
    loop {
        if stop {
            break;
        }
        let _ = stream.read(&mut buf).unwrap();
        for ch in buf.iter() {
            if *ch == b'\0' {
                stop = true;
                break;
            }
            request_array.push(*ch);
        }
        buf = [0; 128];
    }

    let mut result: String = request_array.iter().map(|a| *a as char).collect();
    result = result.replace('\0', "");
    let request = Request::from(result.as_str());
    println!("request: {:?}", request);

    let response = Response::new(HttpStatus::Status200, "hello world\n");
    stream.write(response.to_string().as_bytes()).unwrap();
}
