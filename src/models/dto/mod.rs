pub mod request_dto;

use serde::Serialize;

#[derive(Serialize)]
pub struct APIResponse<T> {
    pub status_code: i32,
    pub message: String,
    pub data: T,
}

impl<T> APIResponse<T> {
    pub fn success(message: &str, data: T) -> Self {
        Self {
            status_code: 200,
            message: message.to_string(),
            data,
        }
    }
}
