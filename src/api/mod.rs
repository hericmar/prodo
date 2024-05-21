use std::fmt::{Display, Formatter};
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use serde_json::json;
use crate::error::{Error, ErrorType};

pub mod controllers;

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self.code {
            ErrorType::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::BadRequest => StatusCode::BAD_REQUEST,
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::Unauthorized => StatusCode::UNAUTHORIZED
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status = self.status_code();
        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => {
                log::error!("{}: {}", status, self.what);

                "Internal Server Error".to_string()
            },
            _ => self.what.clone()
        };

        HttpResponse::build(self.status_code())
            .json(json!({ "error": message }))
    }
}
