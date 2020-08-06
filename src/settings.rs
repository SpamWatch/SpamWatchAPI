use std::path::PathBuf;

use config::{Config, ConfigError, Environment, File};
use dirs::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::utils;

lazy_static! {
    pub static ref ENV: Settings = match Settings::load() {
        Ok(settings) => {
            debug!(utils::LOGGER, "Settings:"; "name" => &settings.database.name);
            settings
        }
        Err(err) => {
            error!(utils::LOGGER, "{}", &format!("{}", err));
            Settings::default()
        }
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct General {
    pub masterid: i64,
    pub token_size: u8,
    pub staging: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseCfg {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerCfg {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub database: DatabaseCfg,
    pub server: ServerCfg,
    pub general: General,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            database: DatabaseCfg {
                host: "127.0.0.1".to_string(),
                port: 5432,
                name: "SpamWatchAPI".to_string(),
                username: "SpamWatchAPI".to_string(),
                password: String::default(),
            },
            server: ServerCfg {
                host: "127.0.0.1".to_string(),
                port: 6345,
            },
            general: General {
                masterid: 777000,
                token_size: 64,
                staging: false,
            },
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, ConfigError> {
        let home_config: PathBuf = match home_dir() {
            Some(path) => [
                path,
                PathBuf::from(&format!(".config/{}/config", &env!("CARGO_PKG_NAME"))),
            ]
                .iter()
                .collect(),
            None => {
                debug!(utils::LOGGER, "Can't get home directory");
                PathBuf::from("config")
            }
        };

        let defaults = Config::try_from(&Settings::default())?;
        let mut settings = Config::default();
        settings.merge(defaults)?;
        settings
            .merge(
                File::with_name(&format!("/etc/{}/config", &env!("CARGO_PKG_NAME")))
                    .required(false),
            )?
            .merge(File::with_name(home_config.to_str().unwrap()).required(false))?
            .merge(File::with_name("config").required(false))?
            .merge(Environment::with_prefix("APP"))?;

        Ok(settings.try_into().unwrap())
    }
}
