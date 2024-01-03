use crate::service::algorithms::Algorithm;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CompressionMetric {
    pub algorithm: String,
    pub input_size: u64,
    pub encoded: String,
    pub decoded: String,
    pub time_taken: Duration,
    pub compression_ratio: f64,
    pub memory_used: u64,
    pub bit_rate: f64,
}

impl CompressionMetric {
    pub fn new(
        algorithm: Algorithm,
        encoded: String,
        decoded: String,
        start_time: Instant,
    ) -> Self {
        println!("Encoded: {:?}", encoded);
        println!("Decoded: {:?}", decoded);
        // if encoded.is_empty() || decoded.is_empty() {
        //     panic!("encoded and decoded text should not be empty");
        // }

        let n_encoded = encoded.len() as f64;
        let n_decoded = decoded.len() as f64;

        let compression_ratio = n_decoded / n_encoded; // ratio of original to encoded text
        let bit_rate = n_encoded / n_decoded; // ratio of encoded text to the original text
        let memory_used = encoded.len() as u64; // amount of bytes used in encoding
        let time_taken = start_time.elapsed(); // amount of time taken to encode and decoded the string
        let input_size = decoded.len() as u64;

        Self {
            algorithm: algorithm.format(),
            input_size,
            encoded,
            decoded,
            time_taken,
            compression_ratio,
            memory_used,
            bit_rate,
        }
    }
}
