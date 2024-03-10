use std::error::Error;
use std::net::TcpStream;
use std::io::Write;
use super::make_http_error::make_http_error;

#[allow(deprecated)]
pub fn write_to_stream(resp: String, mut stream: TcpStream) {
    if let Err(err) = stream.write_all(resp.as_bytes()) {
        make_http_error(err.description(), stream);
        eprintln!("Error writing to stream: {}", err);
    }
}