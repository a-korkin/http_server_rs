use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_bytes(mut stream: TcpStream) {
    let mut buf = [0; 128];

    let mut stop = false;
    let mut count_bytes = 0;
    loop {
        if stop {
            break;
        }
        let _ = stream.read(&mut buf).unwrap();
        for ch in buf.iter() {
            count_bytes += 1;
            if *ch == b'\0' {
                println!("\nstop");
                stop = true;
                break;
            }
            print!("{}", *ch as char);
        }
        buf = [0; 128];
    }

    let mut _request_str: Vec<u8> = vec![0; count_bytes];
    println!("lenght: {}", _request_str.len());

    let response = "HTTP/1.1 200 OK\r\n";
    stream.write(response.as_bytes()).unwrap();
}

fn handle_str(mut stream: TcpStream) {
    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    println!("{}", buf);
    stream.shutdown(std::net::Shutdown::Read).unwrap();
    let response = "HTTP/1.1 200 OK\r\n";
    stream.write(response.as_bytes()).unwrap();
}

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

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_bytes(stream);
        // handle_str(stream);
        // handle_buf(stream);
    }
}
