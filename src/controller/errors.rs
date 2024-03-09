#[derive(Debug)]
pub enum RateLimitingError {
    ExceededMaxRequests,
    FailedToConnect,
}
