use std::io::{Read, Result};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) -> String {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            if let Ok(request) = String::from_utf8(buffer.to_vec()) {
                println!("Received HTTP Request:\n{}", request);
                request
            } else {
                eprintln!("Failed to convert bytes to UTF-8");
                String::from("Invalid UTF-8")
            }
        }
        Err(e) => {
            eprintln!("Error reading from stream: {}", e);
            String::from(format!("Error reading from stream: {}", e))
        }
    }
}
