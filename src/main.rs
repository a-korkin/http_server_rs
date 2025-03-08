use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

mod utils;
use utils::request::Request;
use utils::response::Response;

pub fn parse_request(req: &str) {
    let request = Request::from(req);
    println!("request: {:?}", request);
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
    parse_request(&result);

    let response = Response::new(utils::http::HttpStatus::Status200, "hello");
    stream.write(response.to_string().as_bytes()).unwrap();
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
