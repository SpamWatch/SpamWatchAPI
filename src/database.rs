use postgres::{Client, Config, NoTls};
use slog::Logger;

use crate::settings::Settings;
use crate::utils;
use serde_json::{Value, json};

pub struct Database {
    conn: Client,
    cfg: Settings,
    logger: Logger,
}

impl Database {
    pub fn new() -> Result<Database, Box<std::error::Error>> {
        let logger = utils::logger();
        let cfg = Settings::load()?;
        debug!(logger, "Connecting to database";
         "host" => &cfg.database.host,
         "port" => &cfg.database.port,
         "name" => &cfg.database.name,
         "username" => &cfg.database.username);
        let conn = Config::new()
            .host(&cfg.database.host)
            .port(cfg.database.port)
            .dbname(&cfg.database.name)
            .user(&cfg.database.username)
            .password(&cfg.database.password)
            .application_name(&env!("CARGO_PKG_NAME"))
            .connect(NoTls)?;
        debug!(logger, "Connected to PostgreSQL");
        Ok(Database { conn, cfg, logger })
    }

    pub fn setup_tables(&mut self) -> Result<(), postgres::Error> {
        let create_banlist = "CREATE TABLE IF NOT EXISTS banlist (id integer NOT NULL PRIMARY KEY, reason Text NOT NULL, date timestamp NOT NULL);";
        debug!(self.logger, "Creating Table if it doesn't exist"; "query" => create_banlist, "name" => "banlist");
        self.conn.simple_query(create_banlist)?;

        let create_tokens = "CREATE TABLE IF NOT EXISTS tokens (id SERIAL, token Text NOT NULL PRIMARY KEY, permissions json NOT NULL, userid integer NOT NULL);";
        debug!(self.logger, "Creating Table if it doesn't exist";"query" => create_tokens,  "name" => "tokens");
        self.conn.simple_query(create_tokens)?;
        Ok(())
    }

    pub fn create_genesis_token(&mut self)  -> Result<(), postgres::Error> {
        let logger = utils::logger();
        let get_genesis_token = "SELECT * FROM tokens WHERE id = 1;";
        debug!(self.logger, "Checking if Genesis Token exists"; "query" => get_genesis_token);
        if self.conn.query(get_genesis_token, &[])?.is_empty() {
            info!(self.logger, "Genesis Token doesn't exist. Creating one"; "size" => self.cfg.token_size);
            let token = self.create_token(json!({"all": "rw"}), self.cfg.masterid)?;
            info!(logger, "Created Genesis Token `{}`. Write this down, this will be the only time you see it.", token)
        } else {
            debug!(logger, "Genesis Token exists. Skipping creation.")
        }
        Ok(())
    }

    pub fn create_token(&mut self, permissions: Value, userid: i32) -> Result<String, postgres::Error> {
        let token = nanoid::generate(self.cfg.token_size as usize);
        let insert_token = "INSERT INTO tokens(token, permissions, userid) VALUES ($1, $2, $3);";
        debug!(self.logger, "Creating Token"; "query" => insert_token);
        self.conn.execute(insert_token, &[&token, &permissions, &userid])?;
        Ok(token)
    }
}
