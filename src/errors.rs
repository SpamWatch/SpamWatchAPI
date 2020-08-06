use std::fmt;

use actix_web::error;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use failure::Fail;
use postgres;
use serde_json::{json, Value};

use crate::utils;

#[derive(Fail, Debug)]
pub enum UserError {
    Internal,
    NotFound,
    BadRequest,
    MethodNotAllowed,
    Unauthorized,
    Forbidden,
    TooManyRequests {
        until: i64,
    },
}

impl From<postgres::Error> for UserError {
    fn from(item: postgres::Error) -> Self {
        error!(utils::LOGGER, "{}", item);
        UserError::Internal
    }
}

impl From<serde_json::error::Error> for UserError {
    fn from(item: serde_json::error::Error) -> Self {
        error!(utils::LOGGER, "{}", item);
        UserError::Internal
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {}", self.to_json())
    }
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        self.to_response()
    }

    fn render_response(&self) -> HttpResponse {
        self.to_response()
    }
}

impl UserError {
    fn to_json(&self) -> Value {
        match *self {
            UserError::Internal => json!({
                "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "error": StatusCode::INTERNAL_SERVER_ERROR.canonical_reason()
            }),
            UserError::NotFound => json!({
                "code": StatusCode::NOT_FOUND.as_u16(),
                "error": StatusCode::NOT_FOUND.canonical_reason()
            }),
            UserError::BadRequest => json!({
                "code": StatusCode::BAD_REQUEST.as_u16(),
                "error": StatusCode::BAD_REQUEST.canonical_reason()
            }),
            UserError::MethodNotAllowed => json!({
                "code": StatusCode::METHOD_NOT_ALLOWED.as_u16(),
                "error": StatusCode::METHOD_NOT_ALLOWED.canonical_reason()
            }),
            UserError::Unauthorized => json!({
                "code": StatusCode::UNAUTHORIZED.as_u16(),
                "error": StatusCode::UNAUTHORIZED.canonical_reason()
            }),
            UserError::Forbidden => json!({
                "code": StatusCode::FORBIDDEN.as_u16(),
                "error": StatusCode::FORBIDDEN.canonical_reason()
            }),
            UserError::TooManyRequests { until } => json!({
                "code": StatusCode::TOO_MANY_REQUESTS.as_u16(),
                "error": StatusCode::TOO_MANY_REQUESTS.canonical_reason(),
                "until": until
            }),
        }
    }

    pub fn to_response(&self) -> HttpResponse {
        match *self {
            UserError::Internal => HttpResponse::InternalServerError().json(self.to_json()),
            UserError::NotFound => HttpResponse::NotFound().json(self.to_json()),
            UserError::BadRequest => HttpResponse::BadRequest().json(self.to_json()),
            UserError::MethodNotAllowed => HttpResponse::MethodNotAllowed().json(self.to_json()),
            UserError::Unauthorized => HttpResponse::Unauthorized().json(self.to_json()),
            UserError::Forbidden => HttpResponse::Forbidden().json(self.to_json()),
            UserError::TooManyRequests { until: _ } => HttpResponse::TooManyRequests().json(self.to_json()),
        }
    }
}
