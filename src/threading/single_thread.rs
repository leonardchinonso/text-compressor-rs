use crate::algorithms::Algorithm;
use crate::io::new_codec;
use crate::models::compression_metric::CompressionMetric;
use std::error::Error;
use std::time::Instant;

/// compute_algorithm benchmarks how long a particular algorithm took to run
pub fn compute_algorithm(
    text: String,
    algorithm: Algorithm,
) -> Result<CompressionMetric, Box<dyn Error>> {
    let start_time = Instant::now();

    let mut codec = new_codec(text, algorithm.clone())?;
    codec.encode();
    codec.decode();

    let metric = CompressionMetric::new(
        algorithm,
        codec.compressed(),
        codec.decompressed(),
        start_time,
    );

    Ok(metric)
}
