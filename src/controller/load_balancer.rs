use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;


#[derive(Clone,Debug)]
pub struct LoadBalancer {
    pub load_balancer_ip: String,
    pub health_check_path: String,
    pub health_check_interval: u64,
    pub upstream_servers: Vec<String>,
    pub dead_upstreams: HashSet<String>,
}

impl LoadBalancer {
    pub fn new(load_balancer_ip: String, health_check_path: String, health_check_interval: u64, upstream_servers: Vec<String>) -> LoadBalancer {
        LoadBalancer {
            load_balancer_ip,
            health_check_path,
            health_check_interval,
            upstream_servers,
            dead_upstreams: HashSet::new(),
        }
    }

pub fn connect_to_upstream(&mut self) -> Option<String> {
    if self.upstream_servers.is_empty() {
        return None;
    }

    for i in 0..self.upstream_servers.len() {
        let upstream = &self.upstream_servers[i];
        if self.dead_upstreams.contains(upstream) {
            continue; // Skip dead upstream servers
        }

        let mut stream = match TcpStream::connect(upstream) {
            Ok(stream) => {
                log::info!("Connected to upstream server: {}", upstream);
                stream
            },
            Err(_) => {
                self.mark_as_dead(upstream.clone());
                continue;
            }
        };

        // Perform a simple HTTP GET request to check server health
        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            self.health_check_path, upstream
        );
        if let Err(_) = stream.write_all(request.as_bytes()) {
            log::error!("Failed to send request to upstream server: {}", upstream);
            self.mark_as_dead(upstream.clone());
            continue;
        }

        log::info!("Request sent to upstream server: {}", upstream);

        // Read the response
        let mut response = String::new();
        if let Err(_) = stream.read_to_string(&mut response) {
            log::error!("Failed to receive response from upstream server: {}", upstream);
            self.mark_as_dead(upstream.clone());
            continue;
        }

        log::info!("Received response from upstream server: {}", upstream);

        // Check if the response indicates a healthy server
        if response.contains("200 OK") {
            log::info!("Response indicates a healthy server: {}", upstream);
            return Some(upstream.clone());
        } else {
            self.mark_as_dead(upstream.clone());
            self.print_dead_servers();
        }
    }

    None
}
    // pub fn start_health_checks(&mut self) {
    //     let mut load_balancer_clone = self.clone(); // Clone self to move into the thread

    //     let handle = thread::spawn(move || {
    //         loop {
    //             // Perform health checks
    //             log::info!("Starting health checks...");
    //             match load_balancer_clone.connect_to_upstream() {
    //                 Some(server) => log::info!("Health check successful for server: {}", server),
    //                 None => log::info!("No available upstream servers."),
    //             }

    //             // Sleep for the health check interval
    //             std::thread::sleep(Duration::from_secs(load_balancer_clone.health_check_interval));
    //         }
    //     });

    //     // Join the thread to ensure it terminates before returning
    //     handle.join().expect("Health check thread panicked");
    // }

    pub fn print_dead_servers(&self) {
        println!("Dead Servers:");
        for server in &self.dead_upstreams {
            println!("{}", server);
        }
    }

    pub fn mark_as_dead(&mut self, upstream: String) {
        self.dead_upstreams.insert(upstream);
    }
}