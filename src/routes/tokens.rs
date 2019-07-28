use actix_web::{HttpRequest, Result};
use actix_web::HttpResponse;

use crate::database::Database;
use crate::errors::UserError;
use crate::utils;

pub fn get_tokens() -> Result<HttpResponse, UserError> {
    let logger = utils::logger();
    let mut db = Database::new()
        .map_err(|e| {
            error!(logger, "{}", e);
            UserError::Internal
        })?;
    let tokens = db.get_tokens()
                   .map_err(|e| {
                       error!(logger, "{}", e);
                       UserError::Internal
                   })?;
    let tokens_json = serde_json::to_value(tokens)
        .map_err(|e| {
            error!(logger, "{}", e);
            UserError::Internal
        })?;

    Ok(HttpResponse::Ok().json(tokens_json))
}

//pub fn get_token(path: web::Path<(String, String)>) -> HttpResponse {
//
//}
//
//pub fn create_token() -> HttpResponse {
//
//}
