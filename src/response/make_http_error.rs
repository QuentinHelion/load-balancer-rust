use super::gen::generator;
use super::resp::response;

use std::net::TcpStream;

pub fn make_http_error(error: &str, stream: TcpStream){
    let response_gen = generator("300 OK", "text/plain", error);

    response(response_gen, stream);
}