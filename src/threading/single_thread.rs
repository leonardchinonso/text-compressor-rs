use crate::{
    models::compression_metric::CompressionMetric,
    service::{algorithms::Algorithm, io::new_codec},
};
use std::error::Error;
use std::time::Instant;

/// compute_algorithm benchmarks how long a particular algorithm took to run
pub fn compute_algorithm(text: String, algorithm: Algorithm) -> CompressionMetric {
    let start_time = Instant::now();

    let mut codec = new_codec(text, algorithm.clone()).expect("codec should not be none");
    codec.encode();
    codec.decode();

    let metric = CompressionMetric::new(
        algorithm,
        codec.compressed(),
        codec.decompressed(),
        start_time,
    );

    metric
}
