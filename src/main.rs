#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate slog;
extern crate sloggers;

use crate::database::Database;
use crate::settings::Settings;

mod settings;
mod database;
mod utils;

fn setup_database() {
    let mut postgresql = Database::new();
    postgresql.setup_tables();
    postgresql.create_genesis_token();
}

fn main() {
    let logger = utils::logger();
    info!(logger, "Starting {}", env!("CARGO_PKG_NAME"); "version" => &env!("CARGO_PKG_VERSION"));
    let settings = Settings::load();
    if settings.masterid == 777000 {
        warn!(logger, "MasterID not set. Defaulting to Telegrams id (777000). To avoid this set `masterid` under the `general` section in the config.")
    }
    info!(logger, "Master ID is {}", settings.masterid);
    setup_database()
//    rocket::ignite().mount("/hello", routes![world]).launch();
}
