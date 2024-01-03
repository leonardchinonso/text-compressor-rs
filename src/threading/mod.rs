pub mod multi_thread;
pub mod single_thread;

use crate::{
    models::{compression_metric::CompressionMetric, threader::ThreadType},
    service::{algorithms::Algorithm, pkg::traits::Reader},
};
use std::error::Error;

#[derive(Clone)]
pub struct Threader {}

impl Threader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn benchmark_algorithms(
        &self,
        text: String,
        thread_type: ThreadType,
    ) -> Vec<CompressionMetric> {
        // let mut file = File::new(&file_name, "test_data/out_data.txt");
        // let text = file.read().expect("cannot read file!");

        let algorithms = [
            Algorithm::Rle,
            Algorithm::Lzw,
            Algorithm::Bwt,
            Algorithm::Huffman,
            Algorithm::BwtRle,
        ];

        let mut metrics = Vec::with_capacity(algorithms.len());

        algorithms.into_iter().for_each(|algorithm| {
            metrics.push({
                match thread_type {
                    ThreadType::MultiThreaded => {
                        multi_thread::compute_algorithm(text.clone(), algorithm)
                    }
                    ThreadType::SingleThreaded => {
                        single_thread::compute_algorithm(text.clone(), algorithm)
                    }
                }
            });
        });

        for metric in metrics.iter() {
            if !metric.decoded.is_empty() {
                assert_eq!(metric.decoded, text);
            }
        }

        metrics
    }
}
