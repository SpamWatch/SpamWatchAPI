use std::fmt;

use actix_web::{error};
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use failure::Fail;
use serde_json::json;
use postgres;

use crate::utils;

#[derive(Fail, Debug)]
pub enum UserError {
    Internal,
    NotFound,
    BadRequest
}

impl From<postgres::Error> for UserError {
    fn from(item: postgres::Error) -> Self {
        error!(utils::LOGGER, "{}", item);
        UserError::Internal
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_json = match *self {
            UserError::Internal => {
                json!({
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "error": StatusCode::INTERNAL_SERVER_ERROR.canonical_reason()
                    })
            }
            UserError::NotFound => {
                json!({
                        "code": StatusCode::NOT_FOUND.as_u16(),
                        "error": StatusCode::NOT_FOUND.canonical_reason()
                    })
            },
            UserError::BadRequest => {
                json!({
                        "code": StatusCode::BAD_REQUEST.as_u16(),
                        "error": StatusCode::BAD_REQUEST.canonical_reason()
                    })
            }
        };
        write!(f, "{}", error_json.to_string())
    }
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UserError::Internal => {
                HttpResponse::InternalServerError()
                    .json(self.to_string())
            },
            UserError::NotFound => {
                HttpResponse::NotFound()
                    .json(self.to_string())
            },
            UserError::BadRequest => {
                HttpResponse::BadRequest()
                    .json(self.to_string())
            }
        }
    }
}
