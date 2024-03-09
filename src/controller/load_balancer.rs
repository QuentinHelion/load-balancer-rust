use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct LoadBalancer {
    pub load_balancer_ip: String,
    pub health_check_path: String,
    pub health_check_interval: u64,
    pub upstream_servers: Arc<Vec<String>>,
    pub dead_upstreams: HashSet<String>,
    pub last_selected_index: IndexHolder,
}

// New type to hold Mutex<usize> and implement Clone
#[derive(Clone, Debug)]
pub struct IndexHolder(Arc<Mutex<usize>>);

impl LoadBalancer {
    pub fn new(
        load_balancer_ip: String,
        health_check_path: String,
        health_check_interval: u64,
        upstream_servers: Vec<String>,
    ) -> LoadBalancer {
        LoadBalancer {
            load_balancer_ip,
            health_check_path,
            health_check_interval,
            upstream_servers: Arc::new(upstream_servers),
            dead_upstreams: HashSet::new(),
            last_selected_index: IndexHolder(Arc::new(Mutex::new(0))),
        }
    }

    pub fn connect_to_upstream(&self) -> Option<String> {
        let servers = self.upstream_servers.as_ref();
        if servers.is_empty() {
            return None;
        }

        let num_servers = servers.len();
        let mut attempts = 0;
        let mut index = self.last_selected_index.0.lock().unwrap(); // Lock the mutex for last_selected_index

        loop {
            let upstream = servers[*index].clone();
            match TcpStream::connect(&upstream) {
                Ok(_) => {
                    log::info!("Connected to upstream server: {}", upstream);
                    let selected_upstream = upstream.clone();
                    *index = (*index + 1) % num_servers; // Update the index
                    log::info!("Incremented last_selected_index: {}", *index);
                    return Some(selected_upstream);
                }
                Err(_) => {
                    attempts += 1;
                    if attempts >= num_servers {
                        break;
                    }
                    *index = (*index + 1) % num_servers; // Update the index
                    log::info!("Incremented last_selected_index: {}", *index);
                }
            }
        }

        None
    }

    pub fn start_health_check(&mut self) {
        let interval = Duration::from_secs(self.health_check_interval);
        let mut self_clone = self.clone();

        thread::spawn(move || {
            loop {
                let healthy_servers = self_clone.health_checking();
                self_clone.upstream_servers = Arc::new(healthy_servers);
                thread::sleep(interval);
            }
        });
    }

    pub fn health_checking(&mut self) -> Vec<String> {
    let mut healthy_servers = Vec::new();
    let servers = self.upstream_servers.clone();
    let servers = servers.as_ref();

    let dead_servers = self.dead_upstreams.clone();
    for upstream in servers.iter().chain(&dead_servers) {
        match TcpStream::connect(upstream) {
            Ok(stream) => {
                log::info!("Connected to upstream server: {}", upstream);
                let mut stream = stream;

                let request = format!(
                    "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                    self.health_check_path, upstream
                );
                if let Err(_) = stream.write_all(request.as_bytes()) {
                    log::error!("Failed to send request to upstream server: {}", upstream);
                    self.mark_as_dead(upstream);
                } else {
                    // log::info!("Request sent to upstream server: {}", upstream);

                    let mut response = String::new();
                    if let Err(_) = stream.read_to_string(&mut response) {
                        log::error!("Failed to receive response from upstream server: {}", upstream);
                        self.mark_as_dead(upstream);
                    } else {
                        if response.contains("200 OK") {
                            if self.dead_upstreams.contains(upstream) {
                                log::info!("Server {} is now healthy. Removed from dead servers.", upstream);
                            }
                            log::info!("Server {} is healthy", upstream);
                            healthy_servers.push(upstream.clone());

                            // Remove the server from dead_upstreams if it was previously marked as dead
                            self.dead_upstreams.retain(|server| server != upstream);
                        } else {
                            log::error!("Response indicates an unhealthy server: {}", upstream);
                            self.mark_as_dead(upstream);
                        }
                    }
                }
            }
            Err(_) => {
                log::error!("Failed to connect to upstream server: {}", upstream);
                self.mark_as_dead(upstream);
            }
        }
    }
    self.print_dead_servers();
    healthy_servers
}


    pub fn print_dead_servers(&self) {
        log::warn!("Dead Servers:");
        for server in &self.dead_upstreams {
            println!("{}", server);
        }
    }

    pub fn mark_as_dead(&mut self, upstream: &String) {
        self.dead_upstreams.insert(upstream.to_string());
    }
}
