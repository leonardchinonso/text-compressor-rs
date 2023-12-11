pub mod args;
pub mod file;


use super::{
    algorithms::{
        burrows_wheeler_run_length::BurrowsWheelerRunLength,
        burrows_wheeler_transform::quadratic_log::BurrowsWheelerTransform,
        huffman::Huffman,
        run_length_encoding::RunLengthEncoding,
        lempel_ziv_welch::LempelZivWelch,
        Algorithm
    },
    pkg::traits::Codec
};


// new_codec takes in an input and choice of algorithm and returns the algorithm implementation
pub fn new_codec(text: String, algorithm: Algorithm) -> Result<Box<dyn Codec>, String> {
    match algorithm {
        Algorithm::Rle => Ok(Box::new(RunLengthEncoding::new(text))),
        Algorithm::Huffman => Ok(Box::new(Huffman::new(text))),
        Algorithm::Bwt => Ok(Box::new(BurrowsWheelerTransform::new(text))),
        Algorithm::Lzw => Ok(Box::new(LempelZivWelch::new(text))),
        Algorithm::BwtRle => Ok(Box::new(BurrowsWheelerRunLength::new(text))),
        _ => Err("invalid algorithm".to_string()),
    }
}
