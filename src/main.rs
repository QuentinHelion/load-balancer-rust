mod responder;

use std::net::TcpListener;

use responder::handle_client;
use responder::generator;
use responder::response;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Listening on 127.0.0.1:8080...");

    for stream in listener.incoming() {
        
        match stream {
            
            Ok(stream) => {
                let stream_buff = stream.try_clone().expect("Failed to clone stream");
                std::thread::spawn(|| {
                    handle_client(stream);
                });

                let gene = generator("200 OK", "text/plain", "Hello world!");
                response(gene, stream_buff);
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }
}
