use std::io::{Read, Write};
use std::net::TcpStream;
use crate::controller::load_balancer::LoadBalancer;
use log;

pub fn handle_connection(mut client_stream: TcpStream, load_balancer: &mut LoadBalancer) {
    log::info!("LB ip : {:?}", load_balancer.load_balancer_ip);
    let upstream_server = match load_balancer.connect_to_upstream() {
        Some(server) => server,
        None => {
            // No available upstream servers, send an error response to the client
            let response = "HTTP/1.1 503 Service Unavailable\r\n\r\n";
            if let Err(e) = client_stream.write_all(response.as_bytes()) {
                log::error!("Failed to send error response to client: {}", e);
            }
            return;
        }
    };

    // Connect to the selected upstream server
    let mut upstream_stream = match TcpStream::connect(upstream_server.clone()) {
        Ok(stream) => stream,
        Err(e) => {
            log::error!("Failed to connect to upstream server {}: {}", upstream_server, e);
            // Mark the upstream server as dead
            load_balancer.mark_as_dead(upstream_server.clone());
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
