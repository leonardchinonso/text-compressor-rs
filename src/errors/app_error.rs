use actix_web::HttpResponse;
use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum ErrorKind {
    InternalServerError,
    NotFound,
    FailedAction,
}

// AppError is a custom warehouse application error
#[derive(Debug, Serialize)]
pub struct AppError {
    pub status_code: i32,
    pub message: String,
    pub kind: ErrorKind,
}

impl AppError {
    pub fn new(msg: &str, err_kind: ErrorKind) -> Self {
        let status_code = match err_kind {
            ErrorKind::FailedAction => 400,
            ErrorKind::NotFound => 404,
            ErrorKind::InternalServerError => 500,
        };

        Self {
            status_code,
            message: msg.to_string(),
            kind: err_kind,
        }
    }

    pub fn to_responder(self) -> HttpResponse {
        match self.kind {
            ErrorKind::InternalServerError => HttpResponse::InternalServerError().finish(),
            ErrorKind::NotFound => HttpResponse::BadRequest()
                .reason("resource not found")
                .json(self),
            ErrorKind::FailedAction => HttpResponse::BadRequest().json(self),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.message
    }
}
