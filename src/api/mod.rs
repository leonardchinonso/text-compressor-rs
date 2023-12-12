use actix_web::web;

pub mod threader_router;

// init configures routes for the application
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(threader_router::benchmark_single_thread);
    cfg.service(threader_router::benchmark_multi_thread);
}
