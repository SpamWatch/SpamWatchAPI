use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde::Deserialize;
use serde_json::Value;

use crate::database::Database;
use crate::errors::UserError;
use crate::guards::TokenGuard;
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct CreateBan {
    id: i64,
    reason: String,
    message: Option<String>,
}

pub fn get_bans(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = TokenGuard::new(utils::get_auth_token(&req)?)?;
    if guard.root() {
        let mut db = Database::new()?;
        let bans = db.get_bans()?;
        let nicer_bans: Vec<Value> = bans
            .iter()
            .map(|ban| ban.raw_json())
            .collect();
        let bans_json = serde_json::to_value(nicer_bans).map_err(|e| {
            error!(utils::LOGGER, "{}", e);
            UserError::Internal
        })?;

        Ok(HttpResponse::Ok().json(bans_json))
    } else {
        Err(UserError::Forbidden)
    }
}

pub fn post_bans(
    req: HttpRequest,
    data: web::Json<Vec<CreateBan>>,
) -> Result<HttpResponse, UserError> {
    let guard = TokenGuard::new(utils::get_auth_token(&req)?)?;
    if guard.admin() {
        let mut db = Database::new()?;
        for ban in data.iter() {
            if !ban.reason.is_empty() {
                db.add_ban(ban.id,
                           &ban.reason,
                           guard.token.id,
                           &ban.message)?;
            } else {
                return Err(UserError::BadRequest("ban reason can not be empty"));
            }
        }
        Ok(HttpResponse::NoContent().body(""))
    } else {
        Err(UserError::Forbidden)
    }
}

pub fn get_ban(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let _guard = TokenGuard::new(utils::get_auth_token(&req)?)?;
    let user_id: i64 = req.match_info().get("id").unwrap().parse().map_err(|_| {
        UserError::BadRequest("could not convert user id to integer")
    })?;
    let mut db = Database::new()?;
    match db.get_ban(user_id)? {
        Some(ban) => Ok(HttpResponse::Ok().json(ban.json()?)),
        None => Err(UserError::NotFound),
    }
}

pub fn delete_ban(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = TokenGuard::new(utils::get_auth_token(&req)?)?;
    if guard.admin() {
        let user_id: i64 = req.match_info().get("id").unwrap().parse().map_err(|_| {
            UserError::BadRequest("could not convert id to integer")
        })?;

        let mut db = Database::new()?;

        match db.get_ban(user_id)? {
            Some(_) => {
                db.delete_ban(user_id)?;
                Ok(HttpResponse::NoContent().body(""))
            }
            None => Err(UserError::NotFound),
        }
    } else {
        Err(UserError::Forbidden)
    }
}

pub fn get_bans_id_list(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let mut guard = TokenGuard::new(utils::get_auth_token(&req)?)?;
    guard.banlist_all()?;
    let mut db = Database::new()?;
    let bans = db.get_banned_ids()?;
    let nicer_bans: Vec<&i64> = bans
        .iter()
        .collect();
    let response: Vec<String> = nicer_bans.iter().map(|i| i.to_string()).collect();

    Ok(HttpResponse::Ok().body(response.join("\n")))
}
