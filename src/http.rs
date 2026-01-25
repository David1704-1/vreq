use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// HTTP methods
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl Default for Method {
    fn default() -> Self {
        Method::GET
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
            Method::PATCH => write!(f, "PATCH"),
            Method::HEAD => write!(f, "HEAD"),
            Method::OPTIONS => write!(f, "OPTIONS"),
        }
    }
}

/// HTTP request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: Method::GET,
            url: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}

impl Request {
    pub fn new(method: Method, url: String) -> Self {
        Self {
            method,
            url,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = body;
        self
    }
}

/// HTTP response structure
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub duration_ms: u128,
}

impl Response {
    pub fn new(status: u16, status_text: String, body: String) -> Self {
        Self {
            status,
            status_text,
            headers: HashMap::new(),
            body,
            duration_ms: 0,
        }
    }
}

/// Send an HTTP request and return the response
pub fn send_request(request: &Request) -> Result<Response, Box<dyn std::error::Error>> {
    // TODO: Implement actual HTTP request sending
    // This is where you'll implement the core HTTP functionality using reqwest
    //
    // Steps:
    // 1. Create a reqwest client
    // 2. Build the request with method, URL, headers, and body
    // 3. Send the request and measure duration
    // 4. Parse the response (status, headers, body)
    // 5. Return a Response struct
    //
    // Example skeleton:
    // let client = reqwest::blocking::Client::new();
    // let start = std::time::Instant::now();
    //
    // let mut req_builder = match request.method {
    //     Method::GET => client.get(&request.url),
    //     Method::POST => client.post(&request.url),
    //     Method::PUT => client.put(&request.url),
    //     Method::DELETE => client.delete(&request.url),
    //     Method::PATCH => client.patch(&request.url),
    //     Method::HEAD => client.head(&request.url),
    //     Method::OPTIONS => todo!("OPTIONS not directly supported"),
    // };
    //
    // for (key, value) in &request.headers {
    //     req_builder = req_builder.header(key, value);
    // }
    //
    // if !request.body.is_empty() {
    //     req_builder = req_builder.body(request.body.clone());
    // }
    //
    // let resp = req_builder.send()?;
    // let duration = start.elapsed().as_millis();
    //
    // let status = resp.status().as_u16();
    // let status_text = resp.status().to_string();
    // let body = resp.text()?;
    //
    // Ok(Response {
    //     status,
    //     status_text,
    //     headers: HashMap::new(), // TODO: Parse response headers
    //     body,
    //     duration_ms: duration,
    // })

    // For now, return a placeholder error
    Err("HTTP request sending not yet implemented".into())
}

/// Validate a URL
pub fn is_valid_url(url: &str) -> bool {
    // TODO: Implement URL validation
    // Check if the URL is well-formed
    // Could use url crate or simple regex
    !url.is_empty()
}
