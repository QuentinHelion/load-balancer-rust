mod responder;

use responder::response;
use responder::generator;

use std::net::TcpListener;
use tokio::task;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on 127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept() {
        task::spawn(async {
            let gen_resp = generator("200 OK", "text/plain", "Hello world ");
            response(gen_resp, stream);
        });
    }
}