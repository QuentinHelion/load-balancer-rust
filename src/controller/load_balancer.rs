use crate::controller::rate_limiting::SlidingWindowRateLimiter;
use crate::controller::RateLimitingError;
use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use std::sync::atomic::{AtomicUsize, Ordering};
use crate::response::write_to_stream;
use crate::response::generator;

#[derive(Clone, Debug)]
pub struct LoadBalancer {
    pub load_balancer_ip: String,
    pub health_check_path: String,
    pub health_check_interval: u64,
    pub upstream_servers: Arc<Vec<String>>,
    pub dead_upstreams: HashSet<String>,
    pub last_selected_index: IndexHolder,
    pub rate_limiter: SlidingWindowRateLimiter, // New field for rate limiter
}

// New type to hold Mutex<usize> and implement Clone
#[derive(Debug)]
pub struct IndexHolder {
    index: Arc<AtomicUsize>,
}

impl IndexHolder {
    pub fn new(value: usize) -> Self {
        IndexHolder {
            index: Arc::new(AtomicUsize::new(value)),
        }
    }

    pub fn increment(&self, num_servers: usize) -> usize {
        let index = self.index.fetch_add(1, Ordering::Relaxed) % num_servers;
        index
    }
}

impl Clone for IndexHolder {
    fn clone(&self) -> Self {
        IndexHolder {
            index: self.index.clone(),
        }
    }
}

impl LoadBalancer {
    pub fn new(
        load_balancer_ip: String,
        health_check_path: String,
        health_check_interval: u64,
        upstream_servers: Vec<String>,
        window_size_secs: u64,
        max_requests: u32,
    ) -> LoadBalancer {
        LoadBalancer {
            load_balancer_ip,
            health_check_path,
            health_check_interval,
            upstream_servers: Arc::new(upstream_servers),
            dead_upstreams: HashSet::new(),
            last_selected_index: IndexHolder::new(0),
            rate_limiter: SlidingWindowRateLimiter::new(
                Duration::from_secs(window_size_secs),
                max_requests,
            ), // Initialize rate limiter
        }
    }

    pub fn connect_to_upstream(&self) -> Result<Option<String>, RateLimitingError> {
        let servers = self.upstream_servers.as_ref();
        if servers.is_empty() {
            return Ok(None);
        }

        let num_servers = servers.len();
        let index = self.last_selected_index.increment(num_servers);

        // Select the corresponding upstream server based on the index
        let upstream = servers[index].clone();

        if self.rate_limiter.allow_request() {
            match TcpStream::connect(&upstream) {
                Ok(_) => {
                    log::info!("Connected to upstream server: {}", upstream);
                    Ok(Some(upstream))
                }
                Err(_) => {
                    log::error!("Failed to connect to upstream server: {}", upstream);
                    Err(RateLimitingError::FailedToConnect)
                }
            }
        } else {
            log::warn!("Request denied: Exceeded maximum requests");
            Err(RateLimitingError::ExceededMaxRequests)
        }
    }

    pub fn start_health_check(&mut self) {
        let interval = Duration::from_secs(self.health_check_interval);
        let mut self_clone = self.clone();

        thread::spawn(move || {
            let rate_limiter = self_clone.rate_limiter.clone(); // Clone the rate limiter
            loop {
                // Check and reset rate limiting if needed
                rate_limiter.rate_limit_check(); // Call rate_limit_check on the existing rate limiter

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

                    let request = generator("200", "text/plain", "Connection: close");

                    if let Err(_) = stream.write_all(request.as_bytes()) {
                        log::error!("Failed to send request to upstream server: {}", upstream);
                        self.mark_as_dead(upstream);
                    } else {
                        // log::info!("Request sent to upstream server: {}", upstream);

                        let mut response = String::new();
                        if let Err(_) = stream.read_to_string(&mut response) {
                            log::error!(
                                "Failed to receive response from upstream server: {}",
                                upstream
                            );
                            self.mark_as_dead(upstream);
                        } else {
                            if response.contains("200 OK") {
                                if self.dead_upstreams.contains(upstream) {
                                    log::debug!(
                                        "Server {} is now healthy. Removed from dead servers.",
                                        upstream
                                    );
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
