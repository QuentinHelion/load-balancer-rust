// request.rs

use std::collections::HashMap;
use std::io::{Read, BufReader, Result as IoResult};
use std::net::TcpStream;

#[derive(Debug)]
pub struct ParsedRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

pub async fn read_and_parse_request(stream: &mut TcpStream) -> IoResult<ParsedRequest> {
    let mut buffer = Vec::new();
    let mut reader = BufReader::new(stream);

    // Read data from the stream
    reader.read_to_end(&mut buffer)?;

    // Convert the received bytes to a UTF-8 string
    let request_str = String::from_utf8_lossy(&buffer);

    // Parse the HTTP request
    parse_request(&request_str)
}

pub fn parse_request(request_str: &str) -> IoResult<ParsedRequest> {
    let lines: Vec<&str> = request_str.lines().collect();

    // Ensure there's at least one line in the request
    if let Some(first_line) = lines.get(0) {
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        // Ensure there are at least two parts in the first line
        if parts.len() >= 2 {
            let method = parts[0].to_string();
            let path = parts[1].to_string();

            // Extract headers
            let mut headers = HashMap::new();
            for line in lines.iter().skip(1) {
                if let Some(pos) = line.find(':') {
                    let key = line[..pos].trim().to_string();
                    let value = line[pos + 1..].trim().to_string();
                    headers.insert(key, value);
                }
            }

            // Create the ParsedRequest struct
            let parsed_request = ParsedRequest {
                method,
                path,
                headers,
            };

            // Return the result
            return Ok(parsed_request);
        }
    }

    // Return an error if the request is invalid or incomplete
    Err(tokio::io::Error::new(tokio::io::ErrorKind::InvalidData, "Invalid or incomplete HTTP request"))
}
