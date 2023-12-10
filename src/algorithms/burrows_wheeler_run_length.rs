use crate::pkg::traits::Codec;

use super::{
    burrows_wheeler_transform::quadratic_log::BurrowsWheelerTransform,
    run_length_encoding::RunLengthEncoding,
};

pub struct BurrowsWheelerRunLength {
    bwt: Option<BurrowsWheelerTransform>,
    rle: Option<RunLengthEncoding>,
}

impl BurrowsWheelerRunLength {
    pub fn new(text: String) -> Self {
        let bwt = BurrowsWheelerTransform::new(text);
        Self {
            bwt: Some(bwt),
            rle: None,
        }
    }
}

impl Codec for BurrowsWheelerRunLength {
    /// encode uses double encoding to encode the text data
    fn encode(&mut self) {
        // encode the data using bwt
        let mut bwt = self.bwt.take().unwrap();
        bwt.encode();

        // take the encoded bwt result and encode it using RLE
        let mut rle = RunLengthEncoding::new(bwt.compressed());
        rle.encode();

        self.bwt = Some(bwt);
        self.rle = Some(rle);
    }

    /// decode uses double decoding to decode the compressed data
    fn decode(&mut self) {
        if self.rle.is_none() {
            panic!("Run Length Encoding of the data does not exist");
        }

        // decode the data using RLE
        let mut rle = self.rle.take().unwrap();
        rle.decode();

        // if the RLE decoded data is not the same as the BWT compressed data, panic
        let mut bwt = self.bwt.take().unwrap();
        if rle.decompressed() != bwt.compressed() {
            panic!("RLE decompressed data is not the same as the BWT compressed data");
        }

        // decode the BWT data
        bwt.decode();

        self.rle = Some(rle);
        self.bwt = Some(bwt);
    }

    fn compressed(&self) -> String {
        let rle = self.rle.clone().unwrap();
        rle.compressed()
    }

    fn decompressed(&self) -> String {
        let bwt = self.bwt.clone().unwrap();
        bwt.decompressed()
    }
}
