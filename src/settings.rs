use config::{Config, Environment, File, ConfigError};

#[derive(Debug)]
pub struct DatabaseCfg {
    pub address: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Settings {
    pub database: DatabaseCfg,
    pub masterid: i32,
    pub token_size: u8,
}

impl Settings {

    pub fn load() -> Result<Settings, ConfigError> {
        let mut settings = Config::default();
        settings
            .set_default("database.address", "127.0.0.1")?;
        settings.set_default("database.port", 5432)?;
        settings
            .set_default("database.name", "SpamWatchAPI")?;
        settings
            .set_default("database.username", "SpamWatchAPI")?;
        settings.set_default("general.masterid", 777000)?;
        settings.set_default("general.token_size", 64)?;
        settings
            .merge(File::with_name("config"))?
            .merge(Environment::with_prefix("APP"))?;

        Ok(Settings {
            database: DatabaseCfg {
                address: settings.get::<String>("database.address")?,
                port: settings.get::<u16>("database.port")?,
                name: settings.get::<String>("database.name")?,
                username: settings.get::<String>("database.username")?,
                password: settings.get::<String>("database.password")?,
            },
            masterid: settings.get::<i32>("general.masterid")?,
            token_size: settings.get::<u8>("general.token_size")?,
        })
    }
}
