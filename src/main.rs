mod responder;

use responder::generator;
use responder::{read_and_parse_request, ParsedRequest};
use responder::response;

use std::net::TcpListener;
// use tokio::net::TcpStream;
use tokio::task;

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on 127.0.0.1:8080");

    while let Ok((mut stream, _)) = listener.accept() {

            match read_and_parse_request(&mut stream).await {

                Ok(parsed_request) => {
                    println!("{:?}",parsed_request);
                }
                                 
                Err(err) => {
                    eprintln!("Error parsing request: {}", err);
                }
            }

            

            task::spawn(async {
                let gen_resp = generator("200 OK", "text/plain", "Hello world ");
                response(gen_resp, stream);
            });

    }

    // loop {
    //     if let Ok((mut stream, _)) = listener.accept() {
    //         runtime.block_on(async move {
    //             let mut buffer = Vec::new();

    //             // Read data from the stream
    //             if let Ok(_) = stream.read_to_end(&mut buffer) {
    //                 let request_str = String::from_utf8_lossy(&buffer);

    //                 // Parse the HTTP request using the request.rs module
    //                 match parse_request(&request_str) {
    //                     Ok(parsed_request) => {
    //                         println!("{:?}", parsed_request);

    //                         // Customize the response based on the parsed information
    //                         // let response_body = "Hello, World!";
    //                         // let response = format!(
    //                         //     "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
    //                         //     response_body.len(),
    //                         //     response_body
    //                         // );
    //                         let response_str = "HTTP/1.1 200 OK\r\n";

    //                         response(response_str,stream);

    //                         // Send the HTTP response
    //                         // if let Err(e) = stream.write_all(response.as_bytes()) {
    //                         //     eprintln!("Error sending response: {}", e);
    //                         // }
    //                     }
    //                     Err(err) => {
    //                         eprintln!("Error parsing request: {}", err);
    //                     }
    //                 }
    //             }
    //         });
    //     }
    // }
}
