use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    // str,
};

fn handler(mut stream: TcpStream) {
    let mut buf = [0; 128];

    let mut stop = false;
    loop {
        if stop {
            break;
        }
        let _ = stream.read(&mut buf).unwrap();
        for ch in buf.iter() {
            if *ch == b'\0' {
                println!("\nstop");
                stop = true;
                break;
            }
            print!("{}", *ch as char);
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
