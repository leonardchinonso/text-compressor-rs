use std::error::Error;
use crate::algorithms::Algorithm;
use crate::io::{
    self,
    args::Argument,
    file::File
};
use crate::models::response::Response;
use crate::pkg::traits::Reader;

pub fn benchmark_algorithms(args: Argument) -> Result<Vec<Response>, Box<dyn Error>> {
    let mut file = File::new(&args.file_name(), "test_data/out_data.txt");
    let text = file.read().expect("cannot read file!");

    let algorithms = [Algorithm::Huffman, Algorithm::Lzw, Algorithm::Bwt, Algorithm::Rle];
    let mut responses: Vec<Response> = vec![Response::build(); algorithms.len()];

    algorithms.into_iter().enumerate().for_each(|(i, a)| {
        responses[i] = compute_algorithm(text.clone(), a).unwrap();
    });

    Ok(responses)
}

// TODO: Add benchmarking
/// compute_algorithm benchmarks how long a particular algorithm took to run
fn compute_algorithm(text: String, algorithm: Algorithm) -> Result<Response, Box<dyn Error>> {
    let mut codec = io::new_codec(text, algorithm)?;
    codec.encode();
    codec.decode();

    let mut response = Response::build();
    response.set_encoded(codec.compressed());
    response.set_decoded(codec.decompressed());

    Ok(response)
}

