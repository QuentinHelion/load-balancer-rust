/// WE tried to do some error handling with custom errors.
#[derive(Debug)]
pub enum RateLimitingError {
    /// Indicates that the maximum number of requests has been exceeded.
    ExceededMaxRequests,

    /// Indicates a failure to connect.
    FailedToConnect,
}
