use crate::server::start_server;
use clap::Parser;
use dotenv::dotenv;
use log::info;
use std::env;
use std::error::Error;

mod api;
mod errors;
pub mod models;
mod server;
mod service;
mod threading;
mod utils;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // initialize the environment variable reader
    dotenv().ok();

    // set up env variables for the server
    let host = env::var("HOST").expect("HOST must be set in env");
    let port = env::var("PORT")
        .expect("PORT must be set in env")
        .parse::<u16>()
        .expect("invalid port number");

    // start the server
    start_server(&host, port).await
}
