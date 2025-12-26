use serde::{Deserialize, Serialize};

use crate::wasi::logging::logging::{log, Level};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestData {
    pub first_number: i32,
    pub second_number: i32,
}

pub fn parse_request_data(body: &[u8]) -> Result<RequestData, String> {
    serde_json::from_slice::<RequestData>(body).map_err(|e| {
        log(Level::Error, "", &format!("Failed to parse data: {}", e));
        "Error: Invalid JSON format".to_string()
    })
}
