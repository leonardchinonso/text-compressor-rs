use crate::algorithms::huffman::Huffman;
use crate::algorithms::run_length_encoding::Rle;
use crate::algorithms::Algorithm;

/// Codec is a public trait that holds interfaces for
/// encoding and decoding a vector of characters.
pub trait Codec {
    fn encode(&mut self);
    fn decode(&mut self);
    fn compressed(&self) -> String;
    fn decompressed(&self) -> String;
}

/// Reader is a public trait that holds interfaces for
/// reading text input. Any object that can read text input
/// can implement this method, be it a file reader or an
/// argument parser.
pub trait Reader {
    fn read(&mut self) -> Result<String, Box<dyn std::error::Error>>;
}

/// Writer is a public trait that holds interfaces for
/// writing. Any object that can read text input
/// can implement this method, be it a file reader or an
/// argument parser.
pub trait Writer {
    fn write(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}
