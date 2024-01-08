use std::io::{Error, ErrorKind};
use crate::models::dto::request_dto::{CompressRequest, CompressResponse};
use crate::models::threader::ThreadType;
use crate::server::ServiceManager;


pub fn benchmark_multi_thread(
    request: CompressRequest,
) -> Result<Vec<CompressResponse>, Error> {
    if let Err(err) = request.validate() {
        return Err(Error::new(ErrorKind::InvalidInput, err.message));
    }

    let service_manager = ServiceManager::new();
    let metrics = service_manager.threader.benchmark_algorithms(request.text, ThreadType::MultiThreaded);
    let compress_responses = metrics
        .into_iter()
        .map(|metric| CompressResponse::from(metric))
        .collect::<Vec<CompressResponse>>();

    Ok(compress_responses)
}