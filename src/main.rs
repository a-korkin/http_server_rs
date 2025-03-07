use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

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
    parse_request(&result);

    let response = "HTTP/1.1 200 OK\r\n";
    stream.write(response.as_bytes()).unwrap();
}

#[derive(Debug)]
pub struct Header<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> From<&'a str> for Header<'a> {
    fn from(value: &'a str) -> Self {
        if let Some(res) = value.split_once(":") {
            return Self {
                key: res.0.trim(),
                value: res.1.trim(),
            };
        };

        Self { key: "", value: "" }
    }
}

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
pub struct Request<'a> {
    pub request_line: &'a str,
    pub headers: Vec<Header<'a>>,
    pub body: &'a str,
    pub method: HttpMethod,
    pub target: &'a str,
    pub protocol: &'a str,
}

impl<'a> From<&'a str> for Request<'a> {
    fn from(value: &'a str) -> Self {
        let lines: Vec<&'a str> = value.split("\r\n").collect();
        let mut headers: Vec<Header> = vec![];
        let mut count_headers = 0;
        for header in &lines[1..] {
            count_headers += 1;
            if *header == "" {
                break;
            }
            headers.push(Header::from(*header));
        }
        let request_line = lines[0];
        let rq_tokens: Vec<&str> = request_line.split_whitespace().collect();
        Self {
            request_line,
            headers,
            body: lines[count_headers + 1..][0],
            method: HttpMethod::from(rq_tokens[0]),
            target: rq_tokens[1],
            protocol: rq_tokens[2],
        }
    }
}

fn parse_request(req: &str) {
    let request = Request::from(req);
    println!("request: {:?}", request);
}

#[allow(dead_code)]
fn handle_str(mut stream: TcpStream) {
    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    println!("{}", buf);
    let response = "HTTP/1.1 200 OK\r\n".as_bytes();
    stream.write(response).unwrap();
}

#[allow(dead_code)]
fn handle_buf(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut body = String::new();
    loop {
        let mut buf = String::new();
        let count = reader.read_line(&mut buf).unwrap();
        println!("count: {}", count);
        if buf == "\r\n" {
            println!("empty string");
            while reader.read_to_string(&mut body).unwrap() > 0 {}
            stream.shutdown(std::net::Shutdown::Read).unwrap();
            break;
        }
        if count == 0 {
            break;
        }
    }

    println!("body: {}", body);

    let response = "HTTP/1.1 200 OK\r\n";
    stream.write(response.as_bytes()).unwrap();
}

#[allow(dead_code)]
fn handle(mut stream: TcpStream) {
    let mut buf: Vec<u8> = vec![0];
    let mut reader = BufReader::new(&stream);
    reader.read_until(b'\0', &mut buf).unwrap();

    let response = "HTTP/1.1 200 OK\r\n";
    stream.write(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_request(stream);
        // handle_str(stream);
        // handle_buf(stream);
        // handle(stream);
    }
}
