use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl WasmResponse {
    #[allow(dead_code)]
    pub fn new(status: u16, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        WasmResponse {
            status,
            headers,
            body,
        }
    }

    #[allow(dead_code)]
    pub fn empty() -> Self {
        WasmResponse {
            status: 200,
            headers: HashMap::new(),
            body: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn ok(content: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        WasmResponse {
            status: 200,
            headers,
            body: content.as_bytes().to_vec(),
        }
    }

    #[allow(dead_code)]
    pub fn not_found() -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());

        WasmResponse {
            status: 404,
            headers,
            body: "Not Found".as_bytes().to_vec(),
        }
    }

    #[allow(dead_code)]
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    #[allow(dead_code)]
    pub fn add_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    #[allow(dead_code)]
    pub fn add_status(&mut self, status: u16) {
        self.status = status;
    }

    #[allow(dead_code)]
    pub fn with_header(self, key: &str, value: &str) -> Self {
        let mut headers = self.headers;
        headers.insert(key.to_string(), value.to_string());

        WasmResponse {
            status: self.status,
            headers,
            body: self.body,
        }
    }

    #[allow(dead_code)]
    pub fn with_headers(self, headers: HashMap<String, String>) -> Self {
        WasmResponse {
            status: self.status,
            headers,
            body: self.body,
        }
    }

    #[allow(dead_code)]
    pub fn with_status(self, status: u16) -> Self {
        WasmResponse {
            status,
            headers: self.headers,
            body: self.body,
        }
    }

    #[allow(dead_code)]
    pub fn with_body(self, body: Vec<u8>) -> Self {
        WasmResponse {
            status: self.status,
            headers: self.headers,
            body,
        }
    }

    // etc...
}
