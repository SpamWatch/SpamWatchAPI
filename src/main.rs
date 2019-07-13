#[macro_use]
extern crate slog;

use actix_web::{App, guard, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;

use crate::database::Database;
use crate::settings::Settings;

mod database;
mod utils;
mod routes;
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
    setup_database(Database::new()?)?;

    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(r#" %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %D"#))
            .service(web::resource("/").route(
                web::route()
                    .guard(guard::Get())
                    .to(|| HttpResponse::MethodNotAllowed())
                    .to(routes::root::info)))
            .service(web::resource("/version").route(
                web::route()
                    .guard(guard::Get())
                    .to(|| HttpResponse::MethodNotAllowed())
                    .to(routes::root::version)))
    })
        .bind(format!("{}:{}", cfg.server.host, cfg.server.port))?
        .run()?;
    Ok(())
}
