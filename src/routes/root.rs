use actix_web::HttpResponse;
use serde_json::json;

pub fn info() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(format!(
        "
                <style>* {{font-family: monospace;}}</style>
                {} v{} by {}<br>{}<br><br><a href={}>GitHub</a><br>
                <a href=https://t.me/SpamWatch>Channel</a>",
        &env!("CARGO_PKG_NAME"),
        &env!("CARGO_PKG_VERSION"),
        &env!("CARGO_PKG_AUTHORS"),
        &env!("CARGO_PKG_DESCRIPTION"),
        &env!("CARGO_PKG_REPOSITORY")
    ))
}

pub fn version() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "version": &env!("CARGO_PKG_VERSION"),
        "major": &env!("CARGO_PKG_VERSION_MAJOR"),
        "minor": &env!("CARGO_PKG_VERSION_MINOR"),
        "patch": &env!("CARGO_PKG_VERSION_PATCH")
    }))
}
