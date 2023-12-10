use crate::io::args::Argument;
use clap::Parser;
use env_logger::Builder;
use log::{info, LevelFilter};
use std::error::Error;

mod algorithms;
mod data_structures;
mod io;
mod models;
mod pkg;
mod threading;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    // initialize the logger
    Builder::new().filter(None, LevelFilter::Info).init();

    let mut args = Argument::parse();
    args.validate_file_name()?;

    // log the arguments
    info!("{:?}", args);

    let responses = threading::benchmark_algorithms(args.file_name(), args.should_multithread())?;

    for resp in responses {
        println!("Algorithm: {}", resp.algorithm);
        println!("Time Taken: {:?}", resp.time_taken);
        println!("Compression Ratio: {}", resp.compression_ratio);
        println!("Memory Used: {}", resp.memory_used);
        println!("Input Size: {}", resp.input_size);
        println!("Bit Rate: {}", resp.bit_rate);
        println!();
        println!();
        println!();
    }

    Ok(())
}
