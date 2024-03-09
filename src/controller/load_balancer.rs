use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct LoadBalancer {
    pub load_balancer_ip: String,
    pub health_check_path: String,
    pub health_check_interval: u64,
    pub upstream_servers: Arc<Mutex<Vec<String>>>,
    pub dead_upstreams: HashSet<String>,
}

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
            upstream_servers: Arc::new(Mutex::new(upstream_servers)),
            dead_upstreams: HashSet::new(),
        }
    }

    pub fn connect_to_upstream(&mut self) -> Option<String> {
        let servers = self.upstream_servers.lock().unwrap();
        if servers.is_empty() {
            return None;
        }

        let num_servers = servers.len();
        let mut attempts = 0;
        let mut index = 0; // Start from the first server

        loop {
            let upstream = servers[index].clone();
            match TcpStream::connect(&upstream) {
                Ok(_) => {
                    log::info!("Connected to upstream server: {}", &upstream);
                    return Some(upstream);
                }
                Err(_) => {
                    log::error!("Failed to connect to upstream server: {}", &upstream);
                    index = (index + 1) % num_servers;
                    attempts += 1;
                    if attempts >= num_servers {
                        break;
                    }
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
                let mut servers = self_clone.upstream_servers.lock().unwrap();
                *servers = healthy_servers;
                thread::sleep(interval);
            }
        });
    }

    pub fn health_checking(&mut self) -> Vec<String> {
    let tmp = self.upstream_servers.clone();
    let servers = tmp.lock().unwrap();
    if servers.is_empty() {
        return vec![];
    }

    let mut healthy_servers = Vec::new();

    for upstream in &*servers {
        if !self.dead_upstreams.contains(upstream) {
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
                        log::info!("Request sent to upstream server: {}", upstream);

                        let mut response = String::new();
                        if let Err(_) = stream.read_to_string(&mut response) {
                            log::error!("Failed to receive response from upstream server: {}", upstream);
                            self.mark_as_dead(upstream);
                        } else {
                            log::info!("Received response from upstream server: {}", upstream);
                            if response.contains("200 OK") {
                                log::info!("Response indicates a healthy server: {}", upstream);
                                healthy_servers.push(upstream.clone());
                            } else {
                                self.mark_as_dead(upstream);
                                self.print_dead_servers();
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
    }

    healthy_servers
}

    pub fn print_dead_servers(&self) {
        println!("Dead Servers:");
        for server in &self.dead_upstreams {
            println!("{}", server);
        }
    }
    
    pub fn mark_as_dead(&mut self, upstream: &String) {
        self.dead_upstreams.insert(upstream.to_string());
    }
}
