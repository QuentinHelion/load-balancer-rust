use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SlidingWindowRateLimiter {
    window_size: Duration,
    max_requests: u32,
    requests: Arc<Mutex<HashMap<u64, u32>>>,
    last_window_start: Arc<Mutex<u64>>,
    start_time: Instant,
}

impl SlidingWindowRateLimiter {
    // pub fn get_window_size(&self) -> Duration {
    //     self.window_size
    // }
    // pub fn get_max_requests(&self) -> u32 {
    //     self.max_requests
    // }
    pub fn new(window_size: Duration, max_requests: u32) -> SlidingWindowRateLimiter {
        SlidingWindowRateLimiter {
            window_size,
            max_requests,
            requests: Arc::new(Mutex::new(HashMap::new())),
            last_window_start: Arc::new(Mutex::new(0)),
            start_time: Instant::now(), // Initialize the start time
        }
    }

    pub fn allow_request(&self) -> bool {
        let now_secs = self.start_time.elapsed().as_secs() as u64;
        log::debug!("Value of Instant::now(): {:?}", now_secs);
        let mut requests = self.requests.lock().unwrap();
        let mut last_window_start = self.last_window_start.lock().unwrap();
        let window_start = *last_window_start;

        // If the window has passed, reset the request count
        if now_secs >= window_start + self.window_size.as_secs() {
            *requests = HashMap::new(); // Reset the request count
            *last_window_start = now_secs; // Update the last window start
            println!("New hashmap");
        }

        // Remove timestamps that are outside the current window
        requests.retain(|timestamp, _| *timestamp >= window_start);

        // Count the total number of requests within the window
        let count = requests.values().sum::<u32>();

        println!("Current request count: {}", count);

        if count < self.max_requests {
            // Insert the current timestamp as a new request
            *requests.entry(now_secs).or_insert(0) += 1;
            println!("Request allowed");
            true
        } else {
            println!("Request denied: Exceeded maximum requests");
            false
        }
    }
    pub fn rate_limit_check(&self) {
        let mut requests = self.requests.lock().unwrap();
        let mut last_window_start = self.last_window_start.lock().unwrap();

        let now_secs = Instant::now().elapsed().as_secs() as u64;
        log::info!("Value of now_secs: {:?}", now_secs);
        let window_start = *last_window_start;

        // If the window has passed, reset the request count
        if now_secs >= window_start + self.window_size.as_secs() {
            *requests = HashMap::new(); // Reset the request count
            *last_window_start = now_secs; // Update the last window start
            println!("New hashmap");
            println!("Current request count: 0"); // Print the count immediately after reset
        }

        // Remove timestamps that are outside the current window
        requests.retain(|timestamp, _| *timestamp >= window_start);

        // Count the total number of requests within the window
        let count = requests.values().sum::<u32>();

        println!("Current request count: {}", count);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_rate_limiter_reset() {
        // Create a rate limiter with a window size of 1 second and a maximum of 3 requests
        let rate_limiter = SlidingWindowRateLimiter::new(Duration::from_secs(1), 3);

        // Allow 3 requests within the window size
        assert!(rate_limiter.allow_request());
        assert!(rate_limiter.allow_request());
        assert!(rate_limiter.allow_request());

        // The fourth request should be denied
        assert!(!rate_limiter.allow_request());

        // Wait for the window size to pass
        thread::sleep(Duration::from_secs(1));

        // Allow 3 requests again within the new window
        assert!(rate_limiter.allow_request());
        assert!(rate_limiter.allow_request());
        assert!(rate_limiter.allow_request());

        // The fourth request should still be denied as the window has not reset yet
        assert!(!rate_limiter.allow_request());

        // Wait for another window size to pass
        thread::sleep(Duration::from_secs(1));

        // Allow 3 requests again within the new window
        assert!(rate_limiter.allow_request());
        assert!(rate_limiter.allow_request());
        assert!(rate_limiter.allow_request());
    }
}
