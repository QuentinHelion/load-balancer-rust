/// Generates an HTTP response with the specified status code, content type, and content.
///
/// # Arguments
///
/// * `code` - The HTTP status code to include in the response.
/// * `ct` - The content type of the HTTP response.
/// * `content` - The content to include in the HTTP response.
///
/// # Returns
///
/// A string representing the generated HTTP response.
///
/// # Examples
///
/// ```rust
/// let response = generator("200 OK", "text/plain", "Hello, World!");
/// assert_eq!(response, "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!");
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Notes
///
/// Ensure to provide the appropriate status code, content type, and content to generate a valid
/// HTTP response.
///
/// # Caution
///
/// This function does not validate the provided arguments; it is the responsibility of the
/// user to supply correct values.
/// 
pub fn generator(code: &str, ct: &str,content: &str) -> String{
    format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\n\r\n{}",
        code, ct, content
    )
}