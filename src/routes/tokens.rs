use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde::Deserialize;
use serde_json::Value;

use crate::database::{Database, Token};
use crate::errors::UserError;
use crate::guards::{Permission, PermissionGuard};
use crate::guards::Permission::User;
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct CreateToken {
    id: i32,
    permission: Permission,
}

pub fn get_tokens(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;
    if guard.root() {
        let mut db = Database::new()?;
        let tokens = db.get_tokens()?;
        let tokens_json = serde_json::to_value(tokens)
            .map_err(|e| {
                error!(utils::LOGGER, "{}", e);
                UserError::Internal
            })?;

        Ok(HttpResponse::Ok().json(tokens_json))
    } else {
        Err(UserError::Forbidden)
    }
}


pub fn post_tokens(req: HttpRequest, data: web::Json<CreateToken>) -> Result<HttpResponse, UserError> {
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;
    if guard.root() {
        let mut db = Database::new()?;
        let token = db.create_token(&data.permission, data.id)?;
        match db.get_token(token)? {
            Some(token) => Ok(HttpResponse::Ok().json(token.json()?)),
            None => Err(UserError::NotFound)
        }
    } else {
        Err(UserError::Unauthorized)
    }
}


pub fn get_token(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let mut db = Database::new()?;
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;

    if guard.root() {
        let token_id: i32 = req.match_info().get("id").unwrap().parse().map_err(|e| {
            error!(utils::LOGGER, "{}", e);
            UserError::BadRequest
        })?;
        match db.get_token_by_id(token_id)? {
            Some(token) => Ok(HttpResponse::Ok().json(token.json()?)),
            None => Err(UserError::NotFound)
        }
    } else {
        Err(UserError::Unauthorized)
    }
}

pub fn delete_token(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let mut db = Database::new()?;
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;

    if guard.root() {
        let token_id: i32 = req.match_info().get("id").unwrap().parse().map_err(|e| {
            error!(utils::LOGGER, "{}", e);
            UserError::BadRequest
        })?;
        match db.get_token_by_id(token_id)? {
            Some(token) => {
                db.delete_token_by_id(token_id)?;
                Ok(HttpResponse::NoContent().body(""))
            }
            None => Err(UserError::NotFound)
        }
    } else {
        Err(UserError::Unauthorized)
    }
}


