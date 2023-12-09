use crate::io::args::Argument;
use crate::pkg::traits::{Codec, Reader, Writer};
use clap::Parser;
use env_logger::Builder;
use log::{info, LevelFilter};
use std::error::Error;
use crate::models::response::Response;

mod algorithms;
mod data_structures;
mod io;
mod pkg;
mod utils;
mod single_thread;
mod multi_thread;
mod models;

fn main() -> Result<(), Box<dyn Error>> {
    // initialize the logger
    Builder::new().filter(None, LevelFilter::Info).init();

    let mut args = Argument::parse();
    args.validate_file_name()?;

    // log the arguments
    info!("{:?}", args);

    let mut responses: Vec<Response> = Vec::new();
    if args.is_multithread_on() {
        responses = multi_thread::benchmark_algorithms(args)?;
    } else {
        responses = single_thread::benchmark_algorithms(args.clone())?;
    }

    Ok(())
}
