use std::path::PathBuf;

use config::{Config, ConfigError, Environment, File};
use dirs::home_dir;

use crate::utils;

#[derive(Debug)]
pub struct DatabaseCfg {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct ServerCfg {
    pub host: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct Settings {
    pub database: DatabaseCfg,
    pub server: ServerCfg,
    pub masterid: i32,
    pub token_size: u8,
}

impl Settings {
    pub fn load() -> Result<Settings, ConfigError> {
        let logger = utils::logger();
        let home_config: PathBuf = match home_dir() {
            Some(path) => [path, PathBuf::from(&format!(".config/{}/config", &env!("CARGO_PKG_NAME")))].iter().collect(),
            None => {
                debug!(logger, "Can't get home directory");
                PathBuf::from("config")
            }
        };
        let mut settings = Config::default();
        settings.set_default("database.host", "127.0.0.1")?;
        settings.set_default("database.port", 5432)?;
        settings.set_default("database.name", "SpamWatchAPI")?;
        settings.set_default("database.username", "SpamWatchAPI")?;

        settings.set_default("server.host", "127.0.0.1")?;
        settings.set_default("server.port", 6345)?;

        settings.set_default("general.masterid", 777000)?;
        settings.set_default("general.token_size", 64)?;
        settings
            .merge(File::with_name(&format!("/etc/{}/config",
                                            &env!("CARGO_PKG_NAME")))
                .required(false))?
            .merge(File::with_name(home_config.to_str().unwrap()).required(false))?
            .merge(File::with_name("config").required(false))?
            .merge(Environment::with_prefix("APP"))?;

        Ok(Settings {
            database: DatabaseCfg {
                host: settings.get::<String>("database.host")?,
                port: settings.get::<u16>("database.port")?,
                name: settings.get::<String>("database.name")?,
                username: settings.get::<String>("database.username")?,
                password: settings.get::<String>("database.password")?,
            },
            server: ServerCfg {
                host: settings.get::<String>("server.host")?,
                port: settings.get::<u16>("server.port")?,
            },
            masterid: settings.get::<i32>("general.masterid")?,
            token_size: settings.get::<u8>("general.token_size")?,
        })
    }
}
