use actix_web::{HttpResponse, HttpRequest};
use serde_json::{json, Value};

use crate::{settings, utils};
use crate::errors::UserError;
use crate::guards::TokenGuard;
use crate::database::Database;

fn safe_href(name: &str, url: &str) -> String {
    format!(r#"<a rel="noopener" target="_blank" href="{}" class="white-no-dec-link">{}</a>"#, url, name)
}

pub fn info() -> HttpResponse {
    let staging_prefix = if settings::ENV.general.staging {
        "<h1 style='color: #DE935F'>Staging Instance. This is not the real API.</h1>"
    } else {
        ""
    };
    let body = format!(
        "
        <style>* {{font-family: monospace;}}</style>
        {}
        {} v{} by {}<br>{}<br><br>{}<br>{}<br>{}",
        staging_prefix,
        &env!("CARGO_PKG_NAME"),
        &env!("CARGO_PKG_VERSION"),
        &env!("CARGO_PKG_AUTHORS"),
        &env!("CARGO_PKG_DESCRIPTION"),
        safe_href("GitHub", &env!("CARGO_PKG_REPOSITORY")),
        safe_href("Channel", "https://t.me/SpamWatch"),
        safe_href("Documentation", "https://docs.spamwat.ch")
    );
    HttpResponse::Ok().content_type("text/html").body(body)
}

pub fn version() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "version": &env!("CARGO_PKG_VERSION"),
        "major": &env!("CARGO_PKG_VERSION_MAJOR"),
        "minor": &env!("CARGO_PKG_VERSION_MINOR"),
        "patch": &env!("CARGO_PKG_VERSION_PATCH")
    }))
}

pub fn stats(req: HttpRequest) -> Result<HttpResponse, UserError> {
    let mut db = Database::new()?;
    let total_ban_count = db.get_total_ban_count()?;
    let stats = json!({
        "total_ban_count": total_ban_count
    });
    Ok(HttpResponse::Ok().json(serde_json::to_value(stats)?))
}
