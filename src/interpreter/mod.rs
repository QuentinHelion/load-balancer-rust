/// This module provides utilities for working with HTTP headers.
///
/// The `header` module includes the `Header` struct, representing an HTTP header with a name-value
/// pair.
///
/// # Examples
///
/// ```rust
/// use crate::header::Header;
///
/// let content_type_header = Header {
///     name: String::from("Content-Type"),
///     value: String::from("application/json"),
/// };
///
/// println!("{:?}", content_type_header);
/// ```
///
/// The `header` module can be accessed using `use crate::header;`, and it provides the `Header`
/// struct for working with HTTP headers.
pub mod header;