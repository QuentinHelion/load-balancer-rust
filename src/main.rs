mod response;
mod request;
mod interpreter;

use std::net::TcpListener;

use request::handle_client;
use response::generator;
use response::response;
use interpreter::http_str2struct::HttpRequest;
use response::make_http_error;

use tokio::task;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Listening on 127.0.0.1:8080...");


    while let Ok((stream, _)) = listener.accept() {
        let stream_buff_err = stream.try_clone().expect("Failed to clone stream");
        let stream_buff_res = stream.try_clone().expect("Failed to clone stream");
        let gen_resp = generator("200 OK", "text/plain", "Hello world ");
        task::spawn(async {
            let result = handle_client(stream);

            // Parse the HTTP request string
            match HttpRequest::from_string(result) {
                Ok(parsed_request) => {
                    println!("Parsed HTTP Request: {:#?}", parsed_request);
                }
                Err(err) => {
                    make_http_error(err, stream_buff_err);
                    eprintln!("Error parsing HTTP request: {}", err);
                }
            }
            response(gen_resp, stream_buff_res);
        });
        
    }
}
