use crate::io::{args::Argument, file::File};
use crate::pkg::traits::{Codec, Reader, Writer};
use clap::Parser;
use std::error::Error;

mod algorithms;
mod data_structures;
mod io;
mod pkg;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Argument::parse();
    args.validate_file_name()?;
    // TODO: log the arguments to the console
    println!("{:?}", args);

    let mut pedo_file = File::new(args.file_name().as_str(), "out_data.txt");
    let text = pedo_file.read().expect("cannot read file!");

    let mut codec = io::new_codec(text, args.algorithm())?;
    codec.encode();
    pedo_file.write(codec.compressed().as_bytes())
        .expect("cannot write codec to file!");
    codec.decode();
    pedo_file.write(codec.decompressed().as_bytes())
        .expect("cannot write output to file!");

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
    // read output file and compare the contents
    let mut file2 = File::new("out_data.txt", "");
    file2.read().expect("cannot read output file!");
    assert_eq!(pedo_file, file2);

    Ok(())
}
