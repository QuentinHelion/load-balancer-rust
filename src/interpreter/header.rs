/// Represents an HTTP header with a name-value pair.
///
/// # Examples
///
/// ```rust
/// let header = Header {
///     name: String::from("Content-Type"),
///     value: String::from("application/json"),
/// };
///
/// println!("{:?}", header);
/// ```
#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub value: String,
}