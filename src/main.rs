extern crate postgres;
#[macro_use]
extern crate slog;

use std::process::exit;

use actix_web::{App, HttpServer, web};

use crate::database::Database;
use crate::errors::UserError;

#[macro_use]
mod utils;
mod database;
mod errors;
mod guards;
mod routes;
mod settings;
#[cfg(test)]
mod tests;

fn setup_database() -> Result<i32, postgres::Error> {
    let mut db = match Database::new() {
        Ok(d) => d,
        Err(e) => {
            error!(utils::LOGGER, "A Error occured while connecting to PostgreSQL"; "error" => e.to_string());
            return Ok(1);
        }
    };
    db.create_genesis_token()?;
    Ok(0)
}

fn run() -> Result<i32, postgres::Error> {
    info!(utils::LOGGER, "Starting {}", env!("CARGO_PKG_NAME"); "version" => &env!("CARGO_PKG_VERSION"));
    if settings::ENV.general.masterid == 777000 {
        warn!(utils::LOGGER, "MasterID not set. Defaulting to Telegrams id (777000). To avoid this set `masterid` under the `general` section in the config.")
    }
    info!(
        utils::LOGGER,
        "Master ID is {}",
        settings::ENV.general.masterid
    );
    let db_code = setup_database()?;
    if db_code > 0 {
        return Ok(db_code);
    }
    let location = format!(
        "{}:{}",
        settings::ENV.server.host,
        settings::ENV.server.port
    );
    info!(utils::LOGGER, "Starting Server on {}", location);
    HttpServer::new(|| {
        App::new()
            .default_service(web::route().to(|| UserError::NotFound.to_response()))
            .service(
                web::resource("/")
                    .route(web::get().to(routes::root::info))
                    .route(web::head().to(routes::root::info)),
            )
            .service(web::resource("/version").route(web::get().to(routes::root::version)))
            .service(web::resource("/stats").route(web::get().to(routes::root::stats)))
            .service(
                web::resource("/tokens")
                    .route(web::get().to(routes::tokens::get_tokens))
                    .route(web::post().to(routes::tokens::post_tokens)),
            )
            .service(
                web::resource("/tokens/{id}")
                    .route(web::get().to(routes::tokens::get_token))
                    .route(web::delete().to(routes::tokens::delete_token)),
            )
            .service(
                web::resource("/tokens/userid/{uid}")
                    .route(web::get().to(routes::tokens::get_token_by_userid))
            )
            .service(
                web::resource("/banlist")
                    .route(web::get().to(routes::banlist::get_bans))
                    .route(web::post().to(routes::banlist::post_bans)),
            )
            .service(
                web::resource("/banlist/all")
                    .route(web::get().to(routes::banlist::get_bans_id_list))
            )
            .service(
                web::resource("/banlist/{id}")
                    .route(web::get().to(routes::banlist::get_ban))
                    .route(web::delete().to(routes::banlist::delete_ban)),
            )
    })
        .bind(location)
        .unwrap()
        .run()
        .unwrap();
    Ok(0)
}

fn main() -> Result<(), postgres::Error> {
    let exit_code = run()?;
    exit(exit_code);
}
