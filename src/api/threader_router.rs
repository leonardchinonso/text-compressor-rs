use actix_web::{post, Responder, web::{self, Json}};
use crate::dto::request_dto::CompressRequest;
use crate::server;

#[post("/v1/single-thread")]
pub async fn benchmark_single_thread(
    app_data: web::Data<server::AppState>,
    request: Json<CompressRequest>
) -> impl Responder {
    // if let Err(err) = app_data.service_manager.threader
    // app_data.service_manager.threader.

    // if let Err(err) = app_data.service_manager.threader.
}


fn do_sth(app_data: web::Data<server::AppState>) {
    if let Err(err) = app_data.service_manager.threader.
}