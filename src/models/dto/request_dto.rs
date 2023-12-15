use crate::errors::app_error::{AppError, ErrorKind};
use crate::models::compression_metric::CompressionMetric;
use serde::{Deserialize, Serialize};

// CompressRequest represents the request for compressing a string
#[derive(Deserialize, Debug)]
pub struct CompressRequest {
    pub text: String,
    pub multithread: bool,
}

impl CompressRequest {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.text.is_empty() {
            return Err(AppError::new(
                "text cannot be empty",
                ErrorKind::FailedAction,
            ));
        }
        Ok(())
    }
}

#[derive(Serialize)]
pub struct CompressResponse {
    pub algorithm: String,
    pub input_size: u64,
    pub encoded: String,
    pub decoded: String,
    pub time_taken: u128,
    pub compression_ratio: f64,
    pub memory_used: u64,
    pub bit_rate: f64,
}

impl From<CompressionMetric> for CompressResponse {
    fn from(value: CompressionMetric) -> Self {
        Self {
            algorithm: value.algorithm,
            input_size: value.input_size,
            encoded: value.encoded,
            decoded: value.decoded,
            time_taken: value.time_taken.as_nanos(),
            compression_ratio: value.compression_ratio,
            memory_used: value.memory_used,
            bit_rate: value.bit_rate,
        }
    }
}
