use std::{
    io::BufRead,
    io::{BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    str,
};

fn handler(mut stream: TcpStream) {
    let mut buf = [0; 128];
    let n = stream.read(&mut buf).unwrap();
    println!("bytes: {}\nmsg: {}", n, str::from_utf8(&buf).unwrap());
    let response = "HTTP/1.1 200 OK\r\n";
    stream.write(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handler(stream);
    }
}
