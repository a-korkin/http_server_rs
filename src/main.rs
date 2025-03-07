use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    // str,
};

fn handler(mut stream: TcpStream) {
    let mut buf = [0; 128];

    let mut iter: u8 = 0;
    loop {
        iter += 1;
        let bytes = stream.read(&mut buf).unwrap();
        // println!("{}", str::from_utf8(&buf).unwrap());
        let chars = buf.iter().map(|a| *a as char).collect::<Vec<char>>();
        println!("iter: {}\n{:?}", iter, chars);
        if buf == "".as_bytes() {
            println!("empty");
        }
        if buf == "\r\n".as_bytes() {
            println!("check");
        }
        if bytes == 0 {
            break;
        }
        buf = [0; 128];
    }

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
