use crate::algorithms::burrows_wheeler_run_length::BurrowsWheelerRunLength;
use crate::algorithms::burrows_wheeler_transform::quadratic_log;
use crate::algorithms::huffman::Huffman;
use crate::algorithms::lempel_ziv_welch::LempelZivWelch;
use crate::algorithms::run_length_encoding::RunLengthEncoding;
use crate::algorithms::Algorithm;
use crate::pkg::traits::Codec;

pub mod args;
pub mod file;

// new_codec takes in an input and choice of algorithm and returns the algorithm implementation
pub fn new_codec(text: String, algorithm: Algorithm) -> Result<Box<dyn Codec>, String> {
    match algorithm {
        Algorithm::Rle => Ok(Box::new(RunLengthEncoding::new(text))),
        Algorithm::Huffman => Ok(Box::new(Huffman::new(text))),
        Algorithm::Bwt => Ok(Box::new(quadratic_log::BurrowsWheelerTransform::new(text))),
        Algorithm::Lzw => Ok(Box::new(LempelZivWelch::new(text))),
        Algorithm::BwtRle => Ok(Box::new(BurrowsWheelerRunLength::new(text))),
        _ => Err("invalid algorithm".to_string()),
    }
}
