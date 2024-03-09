use super::header::Header;

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Option<String>,
    pub path: Option<String>,
    pub http_version: Option<String>,
    pub headers: Vec<Header>,
}

impl HttpRequest {
    pub fn from_string(request_str: String) -> Result<HttpRequest, &'static str> {
        let mut lines = request_str.lines();
        
        // Parse the first line to extract method, path, and HTTP version
        let first_line = lines.next().ok_or("Invalid request format")?;
        let mut parts = first_line.split_whitespace();
        let method = parts.next().ok_or("Invalid request format")?.to_string();
        let path = parts.next().ok_or("Invalid request format")?.to_string();
        let http_version = parts.next().ok_or("Invalid request format")?.to_string();

        let mut http_request = HttpRequest {
            method: Some(method),
            path: Some(path),
            http_version: Some(http_version),
            headers: Vec::new(),
        };

        // Parse the headers
        for line in lines {
            if line.is_empty() {
                // Skip empty lines (marks the end of headers)
                break;
            }

            let mut parts = line.splitn(2, ": ");
            let header_name = parts.next().ok_or("Invalid header format")?.to_string();
            let header_value = parts.next().ok_or("Invalid header format")?.to_string();

            match header_name.to_lowercase().as_str() {
                "method" => http_request.method = Some(header_value),
                "path" => http_request.path = Some(header_value),
                "http_version" => http_request.http_version = Some(header_value),
                _ => http_request.headers.push(Header {
                    name: header_name,
                    value: header_value,
                }),
            }
        }

        Ok(http_request)
    }

    pub fn update_structure(&mut self, new_request_str: String) -> Result<(), &'static str> {
        let mut lines = new_request_str.lines();

        // Parse the first line to extract method, path, and HTTP version
        let first_line = lines.next().ok_or("Invalid request format")?;
        let mut parts = first_line.split_whitespace();
        let new_method = Some(parts.next().ok_or("Invalid request format")?.to_string());
        let new_path = Some(parts.next().ok_or("Invalid request format")?.to_string());
        let new_http_version = Some(parts.next().ok_or("Invalid request format")?.to_string());

        // Update method, path, and HTTP version if present in the new request
        if let Some(method) = new_method {
            self.method = Some(method);
        }
        if let Some(path) = new_path {
            self.path = Some(path);
        }
        if let Some(http_version) = new_http_version {
            self.http_version = Some(http_version);
        }

        // Parse the headers and add them to the existing ones
        for line in lines {
            if line.is_empty() {
                // Skip empty lines (marks the end of headers)
                break;
            }

            let mut parts = line.splitn(2, ": ");
            let header_name = parts.next().ok_or("Invalid header format")?.to_string();
            let header_value = parts.next().ok_or("Invalid header format")?.to_string();

            // Check if the header already exists; if not, add it
            if !self.headers.iter().any(|h| h.name == header_name) {
                self.headers.push(Header {
                    name: header_name,
                    value: header_value,
                });
            }
        }

        Ok(())
    }
}
