use crate::algorithms::huffman::Huffman;
use crate::algorithms::run_length_encoding::Rle;
use crate::algorithms::Algorithm;
use crate::pkg::traits::Codec;

pub mod args;
pub mod file;

// new_codec takes in an input and choice of algorithm and returns the algorithm implementation
pub fn new_codec(text: String, algorithm: Algorithm) -> Result<Box<dyn Codec>, String> {
    match algorithm {
        Algorithm::Rle => Ok(Box::new(Rle::new(text))),
        Algorithm::Huffman => Ok(Box::new(Huffman::new(text))),
        Algorithm::Bwt => unimplemented!(),
        Algorithm::Lzw => unimplemented!(),
        Algorithm::Invalid => Err("invalid algorithm".to_string()),
    }
}
