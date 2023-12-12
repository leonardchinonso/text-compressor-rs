pub mod args;
pub mod file;

use super::{
    algorithms::{
        burrows_wheeler_run_length::BurrowsWheelerRunLength,
        burrows_wheeler_transform::quadratic_log::BurrowsWheelerTransform, huffman::Huffman,
        lempel_ziv_welch::LempelZivWelch, run_length_encoding::RunLengthEncoding, Algorithm,
    },
    pkg::traits::Codec,
};

// new_codec takes in an input and choice of algorithm and returns the algorithm implementation
pub fn new_codec(text: String, algorithm: Algorithm) -> Option<Box<dyn Codec>> {
    match algorithm {
        Algorithm::Rle => Some(Box::new(RunLengthEncoding::new(text))),
        Algorithm::Huffman => Some(Box::new(Huffman::new(text))),
        Algorithm::Bwt => Some(Box::new(BurrowsWheelerTransform::new(text))),
        Algorithm::Lzw => Some(Box::new(LempelZivWelch::new(text))),
        Algorithm::BwtRle => Some(Box::new(BurrowsWheelerRunLength::new(text))),
        _ => None,
    }
}
