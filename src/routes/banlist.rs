use actix_web::{HttpRequest, HttpResponse, Result, web};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::database::{Ban, Database, Token};
use crate::errors::UserError;
use crate::guards::{Permission, PermissionGuard};
use crate::guards::Permission::User;
use crate::utils;

#[derive(Debug, Deserialize)]
pub struct CreateBan {
    id: i32,
    reason: String,
}

pub fn get_bans(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;
    if guard.root() {
        let mut db = Database::new()?;
        let bans = db.get_bans()?;
        let mut nicer_bans: Vec<Value> = bans
            .iter()
            .map(|ban| {
                json!({
                    "id": ban.id,
                    "reason": ban.reason,
                    "date": ban.date.timestamp()
                })
            })
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
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;
    if guard.admin() {
        let mut db = Database::new()?;
        for ban in data.iter() {
            if !ban.reason.is_empty() {
                db.add_ban(ban.id, &ban.reason, guard.token.id)?;
            } else {
                return Err(UserError::BadRequest);
            }
        }
        Ok(HttpResponse::NoContent().body(""))
    } else {
        Err(UserError::Forbidden)
    }
}

pub fn get_ban(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;
    let user_id: i32 = req.match_info().get("id").unwrap().parse().map_err(|e| {
        error!(utils::LOGGER, "{}", e);
        UserError::BadRequest
    })?;
    let mut db = Database::new()?;
    match db.get_ban(user_id)? {
        Some(ban) => Ok(HttpResponse::Ok().json(serde_json::to_value(json!({
            "id": ban.id,
            "reason": ban.reason,
            "date": ban.date.timestamp()
        }))?)),
        None => Err(UserError::NotFound),
    }
}

pub fn delete_ban(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let guard = PermissionGuard::new(utils::get_auth_token(&req)?)?;
    if guard.admin() {
        let user_id: i32 = req.match_info().get("id").unwrap().parse().map_err(|e| {
            error!(utils::LOGGER, "{}", e);
            UserError::BadRequest
        })?;

        let mut db = Database::new()?;

        match db.get_ban(user_id)? {
            Some(ban) => {
                db.delete_ban(user_id)?;
                Ok(HttpResponse::NoContent().body(""))
            }
            None => Err(UserError::NotFound),
        }
    } else {
        Err(UserError::Forbidden)
    }
}
