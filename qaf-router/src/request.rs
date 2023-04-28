use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct WasmRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,

    /// The path parameters.
    pub params: HashMap<String, String>,

    /// The query string parameters.
    pub query: HashMap<String, String>,

    /// The environment variables.
    pub env: HashMap<String, String>,
}
