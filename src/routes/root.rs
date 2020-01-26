use actix_web::HttpResponse;
use serde_json::json;

use crate::settings;

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
