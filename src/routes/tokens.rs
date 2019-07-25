use actix_web::{HttpResponse, web};
use actix_web::{error, Result};
use actix_web::http::StatusCode;
use failure::Fail;
use serde_json::json;

use crate::database::Database;
use crate::utils;
use std::fmt;

#[derive(Fail, Debug)]
pub enum UserError {
    InternalError,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", json!({
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "error": StatusCode::INTERNAL_SERVER_ERROR.canonical_reason()
                    }).to_string())
    }
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            UserError::InternalError => {
                HttpResponse::InternalServerError()
                    .json(json!({
                        "code": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "error": StatusCode::INTERNAL_SERVER_ERROR.canonical_reason()
                    }))
            }
        }
    }
}

pub fn get_tokens() -> Result<HttpResponse, UserError> {
    let logger = utils::logger();
    let mut db = Database::new().map_err(|e| {error!(logger, "{}", e); UserError::InternalError})?;
    let tokens = db.get_tokens().map_err(|e| {error!(logger, "{}", e); UserError::InternalError})?;
    let tokens_json = serde_json::to_value(tokens).map_err(|e| {error!(logger, "{}", e); UserError::InternalError})?;

    Ok(HttpResponse::Ok().json(tokens_json))
}

//pub fn get_token(path: web::Path<(String, String)>) -> HttpResponse {
//
//}
//
//pub fn create_token() -> HttpResponse {
//
//}
