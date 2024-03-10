/// This module provides utilities for generating HTTP responses.
///
/// It includes the `generator` function, which can be used to generate HTTP responses with
/// specified status codes, content types, and content.
///
/// # Examples
///
/// ```rust
/// use crate::gen::generator;
///
/// let response = generator("200 OK", "text/plain", "Hello, World!");
/// println!("Generated Response: {}", response);
/// ```
///
/// The `gen` module contains the `generator` function, and it can be accessed using
/// `use crate::gen::generator;`.
mod gen;

pub use gen::generator;