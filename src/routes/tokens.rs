use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde::Deserialize;

use crate::database::Database;
use crate::errors::UserError;
use crate::guards::{Permission, TokenGuard};
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct CreateToken {
    id: i64,
    permission: Permission,
}

pub fn get_tokens(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = TokenGuard::new(utils::get_auth_token(&req)?)?;
    if guard.root() {
        let mut db = Database::new()?;
        let tokens = db.get_tokens()?;
        let tokens_json = serde_json::to_value(tokens).map_err(|e| {
            error!(utils::LOGGER, "{}", e);
            UserError::Internal
        })?;

        Ok(HttpResponse::Ok().json(tokens_json))
    } else {
        Err(UserError::Forbidden)
    }
}

pub fn post_tokens(
    req: HttpRequest,
    data: web::Json<CreateToken>,
) -> Result<HttpResponse, UserError> {
    let guard = TokenGuard::new(utils::get_auth_token(&req)?)?;
    if guard.root() {
        let mut db = Database::new()?;
        let token = db.create_token(&data.permission, data.id)?;
        match db.get_token(token)? {
            Some(token) => Ok(HttpResponse::Created().json(token.json()?)),
            None => Err(UserError::NotFound),
        }
    } else {
        Err(UserError::Forbidden)
    }
}

pub fn get_token(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = TokenGuard::new(utils::get_auth_token(&req)?)?;

    let mut db = Database::new()?;
    let _id = req.match_info().get("id").unwrap();
    if _id == "self" {
        match db.get_token(utils::get_auth_token(&req)?)? {
            Some(token) => Ok(HttpResponse::Ok().json(token.json()?)),
            None => Err(UserError::NotFound),
        }
    } else {
        if guard.root() {
            let token_id: i32 = _id.parse().map_err(|_| {
                UserError::BadRequest("could not convert token id to integer")
            })?;
            match db.get_token_by_id(token_id)? {
                Some(token) => Ok(HttpResponse::Ok().json(token.json()?)),
                None => Err(UserError::NotFound),
            }
        } else {
            Err(UserError::Forbidden)
        }
    }
}

pub fn get_token_by_userid(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = TokenGuard::new(utils::get_auth_token(&req)?)?;

    let mut db = Database::new()?;
    let uid = req.match_info().get("uid").unwrap();

    if guard.root() {
        let uid: i64 = uid.parse().map_err(|_| {
            UserError::BadRequest("could not convert user id to integer")
        })?;
        let tokens = db.get_token_by_userid(uid)?;
        let tokens_json = serde_json::to_value(tokens).map_err(|e| {
            error!(utils::LOGGER, "{}", e);
            UserError::Internal
        })?;

        Ok(HttpResponse::Ok().json(tokens_json))
    } else {
        Err(UserError::Forbidden)
    }
}

pub fn delete_token(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = TokenGuard::new(utils::get_auth_token(&req)?)?;

    if guard.root() {
        let mut db = Database::new()?;
        let token_id: i32 = req.match_info().get("id").unwrap().parse().map_err(|_| {
            UserError::BadRequest("could not convert token id to integer")
        })?;
        match db.get_token_by_id(token_id)? {
            Some(_token) => {
                db.revoke_token_by_id(token_id)?;
                Ok(HttpResponse::NoContent().body(""))
            }
            None => Err(UserError::NotFound),
        }
    } else {
        Err(UserError::Forbidden)
    }
}
