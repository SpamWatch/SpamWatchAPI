use actix_web::HttpRequest;
use lazy_static::lazy_static;
use slog::{Drain, Logger};
use slog_async;
use slog_term;

use crate::errors::UserError;

fn logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().force_color().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    Logger::root(drain, o!())
}

lazy_static! {
    pub static ref LOGGER: Logger = logger();
}

pub fn get_auth_token(req: &HttpRequest) -> Result<String, UserError> {
    let token_header = match req.headers().get("authorization") {
        Some(v) => v.to_str().map_err(|e| {
            error!(LOGGER, "{}", e);
            UserError::BadRequest
        })?,
        None => {
            return Err(UserError::Unauthorized);
        }
    };
    let _token: Vec<&str> = token_header.split_ascii_whitespace().collect();
    Ok(_token.get(1).ok_or(UserError::BadRequest)?.to_string())
}
