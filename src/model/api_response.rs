use reqwest::StatusCode;
use serde_json::Value;

#[derive(Debug)]
pub struct ApiResponse {
    pub status: StatusCode,
    pub data: Value,
}

impl ApiResponse {
    pub fn new(status: StatusCode, data: Value) -> Self {
        ApiResponse { status, data }
    }
}
