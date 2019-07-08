use config::{Config, Environment, File};

#[derive(Debug)]
pub struct Database {
    pub address: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
}


#[derive(Debug)]
pub struct Settings {
    pub database: Database,
    pub masterid: i32,
    pub token_size: u8
}

impl Settings {
    pub fn load() -> Settings {
        let mut settings = Config::default();
        settings.set_default("database.address", "127.0.0.1").unwrap();
        settings.set_default("database.port", 5432).unwrap();
        settings.set_default("database.name", "SpamWatchAPI").unwrap();
        settings.set_default("database.username", "SpamWatchAPI").unwrap();
        settings.set_default("general.masterid", 777000).unwrap();
        settings.set_default("general.token_size", 64).unwrap();
        settings
            .merge(File::with_name("config")).unwrap()
            .merge(Environment::with_prefix("APP")).unwrap();

        Settings {
            database: Database {
                address: settings.get::<String>("database.address").unwrap(),
                port: settings.get::<u16>("database.port").unwrap(),
                name: settings.get::<String>("database.name").unwrap(),
                username: settings.get::<String>("database.username").unwrap(),
                password: settings.get::<String>("database.password").unwrap(),
            },
            masterid: settings.get::<i32>("general.masterid").unwrap(),
            token_size: settings.get::<u8>("general.token_size").unwrap(),
        }
    }
}
