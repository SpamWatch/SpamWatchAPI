#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate slog;
extern crate sloggers;

use rocket::Rocket;

use crate::database::Database;
use crate::settings::Settings;

mod settings;
mod database;
mod utils;
mod routes;
mod response;

fn setup_database() {
    let mut postgresql = Database::new();
    postgresql.setup_tables();
    postgresql.create_genesis_token();
}


fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![routes::root::info, routes::root::version, routes::root::version_part])
}


fn main() {
    let logger = utils::logger();
    info!(logger, "Starting {}", env!("CARGO_PKG_NAME"); "version" => &env!("CARGO_PKG_VERSION"));
    let settings = Settings::load();
    if settings.masterid == 777000 {
        warn!(logger, "MasterID not set. Defaulting to Telegrams id (777000). To avoid this set `masterid` under the `general` section in the config.")
    }
    info!(logger, "Master ID is {}", settings.masterid);
    setup_database();
    rocket().launch();
}
