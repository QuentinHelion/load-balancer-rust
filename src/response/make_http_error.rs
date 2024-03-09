use super::gen::generator;
use super::write_to_stream::write_to_stream;

use std::net::TcpStream;

pub fn make_http_error(error: &str, stream: TcpStream){
    let response_gen = generator("300 OK", "text/plain", error);

    write_to_stream(response_gen, stream);
}