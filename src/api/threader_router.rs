use crate::models::dto::request_dto::CompressResponse;
use crate::models::dto::APIResponse;
use crate::models::{dto::request_dto::CompressRequest, threader::ThreadType};
use crate::server;
use actix_web::{
    post,
    web::{self, Json},
    HttpResponse, Responder,
};
use log::error;

#[post("/v1/single-thread")]
pub async fn benchmark_single_thread(
    app_data: web::Data<server::AppState>,
    request: Json<CompressRequest>,
) -> impl Responder {
    println!("Compress Request: {:?}", request);

    if let Err(err) = request.validate() {
        let err_msg = err.message.clone();
        println!("Request validation failed: {:?}", err_msg);
        return err.to_responder();
    }

    let metrics = app_data
        .service_manager
        .threader
        .benchmark_algorithms(request.text.clone(), ThreadType::SingleThreaded);
    let compress_responses = metrics
        .into_iter()
        .map(|metric| CompressResponse::from(metric))
        .collect::<Vec<CompressResponse>>();

    // return the metric as a response to the client
    HttpResponse::Ok().json(APIResponse::success(
        "metrics retrieved successfully",
        compress_responses,
    ))
}

#[post("/v1/multi-thread")]
pub async fn benchmark_multi_thread(
    app_data: web::Data<server::AppState>,
    request: Json<CompressRequest>,
) -> impl Responder {
    if let Err(err) = request.validate() {
        return err.to_responder();
    }

    let metrics = app_data
        .service_manager
        .threader
        .benchmark_algorithms(request.text.clone(), ThreadType::MultiThreaded);
    let compress_responses = metrics
        .into_iter()
        .map(|metric| CompressResponse::from(metric))
        .collect::<Vec<CompressResponse>>();

    // return the metric as a response to the client
    HttpResponse::Ok().json(APIResponse::success(
        "metrics retrieved successfully",
        compress_responses,
    ))
}
