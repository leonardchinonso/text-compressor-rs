use crate::api;
use crate::threading::Threader;
use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use log::info;
use crate::api::threader_cli::benchmark_multi_thread;
use crate::models::dto::request_dto::CompressRequest;
use crate::service::io::file::File;
use crate::service::pkg::traits::{Reader, Writer};
use std::fs::File as StdFile;

// AppState holds the state of the application
pub struct AppState {
    pub service_manager: ServiceManager,
}

// contains methods for managing the application state
impl AppState {
    pub fn new(service_manager: ServiceManager) -> Self {
        Self { service_manager }
    }
}

// ServiceManager is the struct for managing services
pub struct ServiceManager {
    pub threader: Threader,
}

impl ServiceManager {
    pub fn new() -> Self {
        let threader = Threader::new();
        Self { threader }
    }
}

// start_cli starts the CLI multi-thread for large files
pub fn start_cli(file_name: String) -> Result<(), std::io::Error> {
    let mut file = File::new(&file_name, "out_data.json");
    let text = file.read().expect("cannot read file!");
    let request = CompressRequest::new(text, true);
    let result = benchmark_multi_thread(request);
    let result = match result {
        Ok(result) => result,
        Err(err) => return Err(err),
    };

    let file = StdFile::create("out_data.json")?;
    serde_json::to_writer(file, &result)?;

    Ok(())
}

// start_server starts and launches the http server
pub async fn start_server(host: &str, port: u16) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        // get the handle for the service manager
        let service_manager = ServiceManager::new();

        // initialize cors for the resource gate keeping
        let _cors_middleware = Cors::default()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        // launch the http server
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(AppState::new(service_manager)))
            .configure(api::init)
    })
    .bind((host, port))?
    .run()
    .await
}
