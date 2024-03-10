/// # Main Module
///
/// This module contains the main entry point for the load balancer application.
/// It initializes the necessary components and starts the load balancer server.
///
/// ## Usage
///
/// To run the application, use the following command:
///
/// ```sh
/// cargo run -- --load-balancer-ip 192.168.1.196:5555 -b 192.168.1.192:2222,192.168.1.190:8080,192.168.1.191:3333 -p /health-check -i 3 -s 10 -r 5
/// ```
///
/// For the web servers, we used the actix-web crate to create a simple web server that listens on the specified port and has paths for the health check and the / endpoint. These web servers are hosted on LCX containers created on a Proxmox server.
/// The load balancer test machine is hosted on a separate LCX container on the same Proxmox server.

mod response;
mod interpreter;
mod controller;

use std::net::TcpListener;
use std::thread;
use log;

use controller::{handle_connection, parse_arguments, LoadBalancer};


#[tokio::main]
async fn main() {
    /// Initialize the logger
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    /// Parse CLI arguments
    let args = parse_arguments();

    /// Create a new LoadBalancer instance
    let load_balancer = LoadBalancer::new(
        args.load_balancer_ip.clone(),
        args.health_check_path
            .unwrap_or_else(|| "/health-check".to_string()), // Default health check path
        args.health_check_interval.unwrap_or(60), // Default health check interval of 60 seconds
        args.bind.clone(),
        args.window_size_secs.clone(),
        args.max_requests.clone(),
    );

    /// Bind to the specified address
    let listener = match TcpListener::bind(load_balancer.load_balancer_ip.clone()) {
        Ok(listener) => listener,
        Err(e) => {
            log::error!(
                "Failed to bind to address {}: {}",
                load_balancer.load_balancer_ip,
                e
            );
            return;
        }
    };

    /// Start health check for the load balancer
    let mut load_balancer_clone = load_balancer.clone();
    load_balancer_clone.start_health_check();

    /// Accept incoming connections and spawn threads to handle them
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::info!("New client connection: {}", stream.peer_addr().unwrap());
                log::info!("Index : {:?}", load_balancer.last_selected_index);
                let mut load_balancer_clone2 = load_balancer.clone();
                thread::spawn(move || {
                    handle_connection(stream, &mut load_balancer_clone2);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
