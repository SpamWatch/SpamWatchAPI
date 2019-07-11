use std::env::var;

use rocket::http::Status;
use serde_json::json;

use crate::utils;

#[get("/")]
pub fn info() -> String {
    format!(
        "{} v{} by {}\n{}\n\n{}",
        &env!("CARGO_PKG_NAME"),
        &env!("CARGO_PKG_VERSION"),
        &env!("CARGO_PKG_AUTHORS"),
        &env!("CARGO_PKG_DESCRIPTION"),
        &env!("CARGO_PKG_REPOSITORY")
    )
}

#[get("/version")]
pub fn version() -> String {
    utils::reponse(
        Status::Ok,
        json!({
            "version": &env!("CARGO_PKG_VERSION"),
            "major": &env!("CARGO_PKG_VERSION_MAJOR"),
            "minor": &env!("CARGO_PKG_VERSION_MINOR"),
            "patch": &env!("CARGO_PKG_VERSION_PATCH")
        }),
    )
}

#[get("/version/<part>")]
pub fn version_part(part: String) -> String {
    match var(format!("CARGO_PKG_VERSION_{}", part.to_uppercase())) {
        Ok(version) => utils::reponse(Status::Ok,
                                      json!({"version": {part: version}})),
        Err(_) => utils::error(Status::NotFound,
                               format!("Part `{}` not found", part)),
    }
}
