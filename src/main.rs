mod responder;

use std::net::TcpListener;
use std::net::TcpStream;

use responder::handle_client;
use responder::generator;
use responder::response;

use tokio::task;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Listening on 127.0.0.1:8080...");


    while let Ok((stream, _)) = listener.accept() {
        let stream_buff = stream.try_clone().expect("Failed to clone stream");
        let gen_resp = generator("200 OK", "text/plain", "Hello world ");
        task::spawn(async {
            handle_client(stream);
            response(gen_resp, stream_buff);
        });
        
    }
}
