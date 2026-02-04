use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Method {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
            Method::PATCH => write!(f, "PATCH"),
        }
    }
}

impl From<String> for Method {
    fn from(value: String) -> Method {
        match value.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            _ => Method::GET
        }
    }
}

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

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub duration_ms: u128,
}

impl Default for Response {
    fn default() -> Self {
        Response {
            status: u16::MAX,
            status_text: String::from("Invalid Request"),
            headers: HashMap::new(),
            body: String::from(""),
            duration_ms: 0,
        }
    }
}

pub fn send_request(request: &Request) -> Result<Response, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let start = std::time::Instant::now();

    let mut req_builder = match request.method {
        Method::GET => client.get(&request.url),
        Method::POST => client.post(&request.url),
        Method::PUT => client.put(&request.url),
        Method::DELETE => client.delete(&request.url),
        Method::PATCH => client.patch(&request.url),
    };

    let mut headers = HashMap::new();

    for (key, value) in &request.headers {
        req_builder = req_builder.header(key, value);
        headers.insert(key.to_string(), value.to_string());
    }

    if !request.body.is_empty() {
        req_builder = req_builder.body(request.body.clone());
    }

    let resp = req_builder.send()?;
    let duration = start.elapsed().as_millis();

    let status = resp.status().as_u16();
    let status_text = resp.status().to_string();
    let body = resp.text()?;

    Ok(Response {
        status,
        status_text,
        headers,
        body,
        duration_ms: duration,
    })
}
