#[macro_use]
extern crate slog;

use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use config::ConfigError;
use sloggers;
use sloggers::types::Severity::Critical;

use crate::database::Database;
use crate::settings::Settings;

mod database;
mod utils;
//mod routes;
mod settings;

#[cfg(test)]
mod tests;

fn setup_database(mut postgresql: Database) -> Result<(), postgres::Error> {
    postgresql.setup_tables()?;
    postgresql.create_genesis_token()?;
    Ok(())
}


fn main() -> Result<(), Box<std::error::Error>> {
    let logger = utils::logger();
    info!(logger, "Starting {}", env!("CARGO_PKG_NAME"); "version" => &env!("CARGO_PKG_VERSION"));
    let cfg = Settings::load()?;
    if cfg.masterid == 777000 {
        warn!(logger, "MasterID not set. Defaulting to Telegrams id (777000). To avoid this set `masterid` under the `general` section in the config.")
    }
    info!(logger, "Master ID is {}", cfg.masterid);
    setup_database(Database::new(cfg)?)?;

    fn greet(req: HttpRequest) -> impl Responder {
        let name = req.match_info().get("name").unwrap_or("World");
        format!("Hello {}!", &name)
    }

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();
    Ok(())
}
