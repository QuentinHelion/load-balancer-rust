mod controller;

use controller::{LoadBalancer, handle_connection, parse_arguments};
use std::net::TcpListener;
use std::thread;
use log;

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();
    
    let args = parse_arguments();
    
    let load_balancer = LoadBalancer::new(
        args.load_balancer_ip.clone(),
        args.health_check_path.unwrap_or_else(|| "/health-check".to_string()), // Default health check path
        args.health_check_interval.unwrap_or(60), // Default health check interval of 60 seconds
        args.bind.clone(),
    );


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
    log::info!("Server listening on {}", load_balancer.load_balancer_ip);
    let mut load_balancer_clone = load_balancer.clone();
    load_balancer_clone.start_health_check();
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
