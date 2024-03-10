
use super::load_balancer::LoadBalancer;
use super::RateLimitingError;
use log;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::response::generator;

/// Handles a client connection by forwarding the request to an upstream server.
///
/// If no upstream servers are available, it sends a 503 Service Unavailable response to the client.
/// If the maximum number of requests has been exceeded, it sends a 429 Too Many Requests response to the client.
/// If there is a failure to connect to an upstream server, it retries connecting to another upstream server.
/// If the connection to the upstream server is successful, it forwards the client request to the upstream server and then forwards the upstream server response to the client.
/// We have explicit logs as well for the different stages of the connection handling.


pub fn handle_connection(mut client_stream: TcpStream, load_balancer: &mut LoadBalancer) {
    log::info!(
        "Received new connection from client: {}",
        client_stream.peer_addr().unwrap()
    );

    // Retrieve the upstream server to forward the request to
    let upstream_server = match load_balancer.connect_to_upstream() {
        Ok(Some(server)) => server,
        Ok(None) => {
            // No available upstream servers, send an error response to the client
            let response = generator("503", "text/plain", "Service Unavailable");
            if let Err(e) = client_stream.write_all(response.as_bytes()) {
                log::error!("Failed to send error response to client: {}", e);
            }
            return;
        }
        Err(err) => {
            // Handle the RateLimitingError
            match err {
                RateLimitingError::ExceededMaxRequests => {
                    // Send 429 Too Many Requests status code to the client
                    let response = generator("429", "text/plain", "Too Many Requests");
                    if let Err(e) = client_stream.write_all(response.as_bytes()) {
                        log::error!("Failed to send 429 response to client: {}", e);
                    }
                    return;
                }
                RateLimitingError::FailedToConnect => {
                    println!("Failed to connect to upstream server");
                    // Handle the error case appropriately, e.g., return an error response
                    return;
                }
            }
        }
    };

    // Connect to the selected upstream server
    let mut upstream_stream = match TcpStream::connect(upstream_server.clone()) {
        Ok(stream) => stream,
        Err(e) => {
            log::error!(
                "Failed to connect to upstream server {}: {}",
                upstream_server,
                e
            );
            // Mark the upstream server as dead
            load_balancer.mark_as_dead(&upstream_server);
            // Retry connecting to another upstream server
            handle_connection(client_stream, load_balancer);
            return;
        }
    };

    // Forward client request to upstream server
    let mut buffer = [0; 1024];
    let bytes_read = match client_stream.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            log::error!("Error reading client request: {}", e);
            return;
        }
    };

    if let Err(e) = upstream_stream.write_all(&buffer[..bytes_read]) {
        log::error!("Error sending request to upstream server: {}", e);
        return;
    }

    // Forward upstream server response to client
    let mut upstream_response = [0; 1024];
    let bytes_read = match upstream_stream.read(&mut upstream_response) {
        Ok(n) => n,
        Err(e) => {
            log::error!("Error reading response from upstream server: {}", e);
            return;
        }
    };

    if let Err(e) = client_stream.write_all(&upstream_response[..bytes_read]) {
        log::error!("Error sending response to client: {}", e);
        return;
    }
}
