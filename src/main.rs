use std::env;
use dotenv::dotenv;
use crate::server::start_server;
use clap::Parser;
use std::error::Error;


mod threading;
mod utils;
mod server;
mod service;
mod api;
mod dto;
mod errors;


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // initialize the environment variable reader
    dotenv().ok();

    // set up env variables for the server
    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set in env");
    let port = env::var("PORT").expect("PORT must be set in env").parse::<u16>().expect("invalid port number");

    // start the server
    start_server(server_address, port).await


    // // initialize the logger
    // Builder::new().filter(None, LevelFilter::Info).init();
    //
    // let mut args = Argument::parse();
    // args.validate_file_name()?;
    //
    // // log the arguments
    // info!("{:?}", args);
    //
    // let responses = threading::benchmark_algorithms(args.file_name(), args.should_multithread())?;
    //
    // for resp in responses {
    //     println!("Algorithm: {}", resp.algorithm);
    //     println!("Time Taken: {:?}", resp.time_taken);
    //     println!("Compression Ratio: {}", resp.compression_ratio);
    //     println!("Memory Used: {}", resp.memory_used);
    //     println!("Input Size: {}", resp.input_size);
    //     println!("Bit Rate: {}", resp.bit_rate);
    //     println!();
    //     println!();
    //     println!();
    // }

    // Ok(())
}
