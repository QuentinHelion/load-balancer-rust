use std::net::TcpStream;
use std::io::{Read, Write};

pub fn response(resp: String, mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    stream.write_all(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}