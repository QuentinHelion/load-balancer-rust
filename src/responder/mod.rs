mod response;
mod gen;
mod request;
pub mod http_str2struct;
pub mod header;

pub use response::response;
pub use gen::generator;
pub use request::handle_client;
// pub use http_str2struct::HttpRequest;