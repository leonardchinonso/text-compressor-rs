use crate::io::{args::Argument, file::File};
use crate::pkg::traits::{Codec, Reader, Writer};
use clap::Parser;
use env_logger::Builder;
use log::{info, LevelFilter};
use std::error::Error;

mod algorithms;
mod data_structures;
mod io;
mod pkg;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    // initialize the logger
    Builder::new().filter(None, LevelFilter::Info).init();

    let mut args = Argument::parse();
    args.validate_file_name()?;

    // log the arguments
    info!("{:?}", args);

    let mut file = File::new(&args.file_name(), "test_data/out_data.txt");
    let text = file.read().expect("cannot read file!");

    let mut codec = io::new_codec(text, args.algorithm())?;
    codec.encode();
    file.write(codec.compressed().as_bytes())
        .expect("cannot write codec to file!");
    codec.decode();
    file.write(codec.decompressed().as_bytes())
        .expect("cannot write output to file!");
    //
    // // read output file and compare the contents
    // let mut file2 = File::new("test_data/out_data.txt", "");
    // file2.read().expect("cannot read output file!");
    // assert_eq!(file, file2);

    Ok(())
}
