use hyper::{Body, Request};
use std::collections::HashMap;

// Define a generic struct to represent the parsed request
#[derive(Debug)]
pub struct ParsedRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
}

// Define a function to parse the request into the generic struct
pub async fn parse_request(req: Request<Body>) -> ParsedRequest {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    // Extract headers
    let headers: HashMap<String, String> = req
        .headers()
        .iter()
        .map(|(name, value)| (name.as_str().to_string(), value.to_str().unwrap_or("").to_string()))
        .collect();

    // Extract query parameters
    let query_params: HashMap<String, String> = req
        .uri()
        .query()
        .map(|query| {
            url::form_urlencoded::parse(query.as_bytes())
                .into_owned()
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_default();

    ParsedRequest {
        method,
        path,
        headers,
        query_params,
    }
}
