use gen::generator;
use resp::response;

pub fn make_http_error(error: &str, mut stream: TcpStream){
    let response = generator("300 OK", "text/plain", error);

    Ok(response, stream);
}