pub mod multi_thread;
pub mod single_thread;

use crate::service::{
    algorithms::Algorithm,
    io::file::File,
    models::compression_metric::CompressionMetric,
    pkg::traits::Reader,
};
use std::error::Error;


pub struct Threader;

impl Threader {
    pub fn new() -> Self {
        Self
    }

    pub fn benchmark_algorithms(
        text: String,
        is_multithread: bool,
    ) -> Result<Vec<CompressionMetric>, Box<dyn Error>> {
        // let mut file = File::new(&file_name, "test_data/out_data.txt");
        // let text = file.read().expect("cannot read file!");

        let algorithms = [
            Algorithm::Rle,
            Algorithm::Lzw,
            Algorithm::Bwt,
            Algorithm::Huffman,
            Algorithm::BwtRle,
        ];

        let mut metrics: Vec<CompressionMetric> = Vec::with_capacity(algorithms.len());

        algorithms.into_iter().for_each(|algorithm| {
            metrics.push({
                if is_multithread {
                    multi_thread::compute_algorithm(text.clone(), algorithm).unwrap()
                } else {
                    single_thread::compute_algorithm(text.clone(), algorithm).unwrap()
                }
            });
        });

        for metric in metrics.iter() {
            assert_eq!(metric.decoded, text);
        }

        Ok(metrics)
    }
}
