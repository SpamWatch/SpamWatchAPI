use postgres::{Client, Config, NoTls};
use slog::Logger;

use crate::settings::Settings;
use crate::utils;

pub struct Database {
    conn: Client,
    settings: Settings,
    logger: Logger,
}

impl Database {
    pub fn new() -> Database {
        let settings = Settings::load();
        let logger = utils::logger();

        debug!(logger, "Connecting to database";
         "host" => &settings.database.address,
         "port" => &settings.database.port,
         "name" => &settings.database.name,
         "username" => &settings.database.username);
        let conn = Config::new()
            .host(&settings.database.address)
            .port(settings.database.port)
            .dbname(&settings.database.name)
            .user(&settings.database.username)
            .password(&settings.database.password)
            .application_name(&env!("CARGO_PKG_NAME"))
            .connect(NoTls).unwrap();
        debug!(logger, "Connected to PostgreSQL");
        Database {
            conn,
            settings,
            logger,
        }
    }

    pub fn setup_tables(&mut self) {
        let create_banlist = "CREATE TABLE IF NOT EXISTS banlist (id integer NOT NULL PRIMARY KEY, reason Text NOT NULL, date timestamp NOT NULL);";
        debug!(self.logger, "Creating Table"; "query" => create_banlist, "name" => "banlist");
        self.conn.simple_query(create_banlist).unwrap();

        let create_tokens = "CREATE TABLE IF NOT EXISTS tokens (id SERIAL, token Text NOT NULL PRIMARY KEY, permissions json NOT NULL, userid integer NOT NULL);";
        debug!(self.logger, "Creating Table";"query" => create_tokens,  "name" => "tokens");
        self.conn.simple_query(create_tokens).unwrap();
    }

    pub fn create_genesis_token(&mut self) {
        let logger = utils::logger();
        let get_genesis_token = "SELECT * FROM tokens WHERE id = 1;";
        debug!(self.logger, "Checking if Genesis Token exists"; "query" => get_genesis_token);
        if self.conn.query(get_genesis_token, &[]).unwrap().is_empty() {
            info!(self.logger, "Genesis Token doesn't exist. Creating one"; "size" => self.settings.token_size);
            let token = self.create_token(r#"{"all": "rw"}"#, self.settings.masterid);
            info!(logger, "Created Genesis Token `{}`. Write this down, this will be the only time you see it.", token.unwrap())
        } else {
            debug!(logger, "Genesis Token exists. Skipping creation.")
        }
    }

    pub fn create_token(&mut self, permissions: &str, userid: i32) -> Option<String> {
        let perms: serde_json::Value = serde_json::from_str(&permissions).unwrap();
        let token = nanoid::generate(self.settings.token_size as usize);
        let insert_token = r#"INSERT INTO tokens(token, permissions, userid) VALUES ($1, $2, $3);"#;
        debug!(self.logger, "Creating Token"; "query" => insert_token);
        self.conn.execute(insert_token, &[&token, &perms, &userid]).unwrap();
        Some(token)
    }
}
