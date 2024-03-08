use std::net::TcpStream;
use std::io::{Read, Write};

pub fn response(resp: String, mut stream: TcpStream) {
    stream.write_all(resp.as_bytes()).unwrap();
}