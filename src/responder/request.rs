use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            if let Ok(request) = String::from_utf8(buffer.to_vec()) {
                println!("Received HTTP Request:\n{}", request);
            } else {
                eprintln!("Failed to convert bytes to UTF-8");
            }
        }
        Err(e) => eprintln!("Error reading from stream: {}", e),
    }
}