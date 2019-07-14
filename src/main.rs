#[macro_use]
extern crate slog;

use actix_web::{App, guard, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;

use crate::database::Database;

#[macro_use]
mod utils;
mod settings;
mod database;

mod routes;
#[cfg(test)]
mod tests;

fn setup_database(mut postgresql: Database) -> Result<(), Box<std::error::Error>> {
    postgresql.setup_tables()?;
    postgresql.create_genesis_token()?;
    Ok(())
}


fn main() -> Result<(), Box<std::error::Error>> {
    let logger = utils::logger();
    info!(logger, "Starting {}", env!("CARGO_PKG_NAME"); "version" => &env!("CARGO_PKG_VERSION"));
    if config!(masterid) == 777000 {
        warn!(logger, "MasterID not set. Defaulting to Telegrams id (777000). To avoid this set `masterid` under the `general` section in the config.")
    }
    info!(logger, "Master ID is {}", config!(masterid));
    setup_database(Database::new()?)?;

    let location = format!("{}:{}", config!(server.host), config!(server.port));
    info!(logger, "Starting Server on {}", location);
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(
                web::route()
                    .guard(guard::Any(guard::Get()).or(guard::Head()))
                    .to(|| HttpResponse::MethodNotAllowed())
                    .to(routes::root::info)))
            .service(web::resource("/version").route(
                web::route()
                    .guard(guard::Get())
                    .to(|| HttpResponse::MethodNotAllowed())
                    .to(routes::root::version)))
    })
        .bind(location)?
        .run()?;
    Ok(())
}
