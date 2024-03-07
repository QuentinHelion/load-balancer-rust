mod response;
mod gen;
mod request;

pub use response::response;
pub use gen::generator;
pub use request::read_and_parse_request;
pub use request::ParsedRequest;