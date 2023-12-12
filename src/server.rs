use crate::api;
use crate::threading::Threader;
use actix_cors::Cors;
use actix_web::{http, middleware, web, App, HttpServer};
use log::info;

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
