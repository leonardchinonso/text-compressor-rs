use crate::algorithms::huffman::Huffman;
use crate::algorithms::lempel_ziv_welch::LempelZivWelch;
use crate::algorithms::run_length_encoding::Rle;
use crate::io::file::File;
use crate::pkg::traits::{Codec, Reader, Writer};

mod algorithms;
mod data_structures;
mod io;
mod pkg;

fn main() {
    let mut file = File::new("test_data/book1.txt", "test_data/out_data.txt");
    let text = file.read().expect("cannot read file!");

    // let mut rle = RLE::new(text);
    // rle.encode();
    // file.write(rle.compressed().as_bytes()).expect("cannot write codec to file!");
    // rle.decode();
    // file.write(rle.decompressed().as_bytes()).expect("cannot write output to file!");

    // let mut huffman = Huffman::new(text);
    // huffman.encode();
    // file.write(huffman.compressed().as_bytes())
    //     .expect("cannot write codec to file!");
    // huffman.decode();
    // file.write(huffman.decompressed().as_bytes())
    //     .expect("cannot write output to file!");
    //
    // // read output file and compare the contents
    // let mut file2 = File::new("out_data.txt", "");
    // file2.read().expect("cannot read output file!");
    // assert_eq!(file, file2);

    let mut lzw = LempelZivWelch::new(text);
    lzw.encode();
    file.write(lzw.compressed().as_bytes())
        .expect("cannot write codec to file!");
    lzw.decode();
    file.write(lzw.decompressed().as_bytes())
        .expect("cannot write output to file!");

    // read output file and compare the contents
    let mut file2 = File::new("test_data/out_data.txt", "");
    file2.read().expect("cannot read output file!");
    assert_eq!(file, file2);
}
