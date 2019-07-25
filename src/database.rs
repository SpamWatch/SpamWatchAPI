use postgres::{Client, Config, NoTls, Row};
use serde::Serialize;
use serde_json::{json, Value};
use slog::Logger;

use crate::settings;
use crate::utils;

pub struct Database {
    conn: Client,
    logger: Logger,
}

#[derive(Debug, Serialize)]
pub struct Token {
    id: i32,
    token: String,
    permissions: Value,
    userid: i32,
}

impl Database {
    pub fn new() -> Result<Database, Box<std::error::Error>> {
        let logger = utils::logger();
        debug!(logger, "Connecting to database";
         "host" => config!(database.host),
         "port" => config!(database.port),
         "name" => config!(database.name),
         "username" => config!(database.username));
        let conn = Config::new()
            .host(&config!(database.host))
            .port(config!(database.port))
            .dbname(&config!(database.name))
            .user(&config!(database.username))
            .password(&config!(database.password))
            .application_name(&env!("CARGO_PKG_NAME"))
            .connect(NoTls)?;
        debug!(logger, "Connected to PostgreSQL");
        Ok(Database { conn, logger })
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

    pub fn create_genesis_token(&mut self) -> Result<(), Box<std::error::Error>> {
        let get_genesis_token = "SELECT * FROM tokens WHERE id = 1;";
        debug!(self.logger, "Checking if Genesis Token exists"; "query" => get_genesis_token);
        if self.conn.query(get_genesis_token, &[])?.is_empty() {
            info!(self.logger, "Genesis Token doesn't exist. Creating one"; "size" => config!(token_size));
            let token = self.create_token(json!({"all": "rw"}), config!(masterid))?;
            info!(self.logger, "Created Genesis Token `{}`. Write this down, this will be the only time you see it.", token)
        } else {
            debug!(self.logger, "Genesis Token exists. Skipping creation.")
        }
        Ok(())
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, Box<std::error::Error>> {
        let get_all_tokens = "SELECT * FROM tokens;";
        debug!(self.logger, "Getting all tokens"; "query" => get_all_tokens);
        let result: Vec<Row> = self.conn.query(get_all_tokens, &[])?;
        Ok(result.into_iter()
                 .map(|row| Token {
                     id: row.get(0),
                     token: row.get(1),
                     permissions: row.get(2),
                     userid: row.get(3),
                 })
                 .collect())
    }

    pub fn create_token(&mut self, permissions: Value, userid: i32) -> Result<String, Box<std::error::Error>> {
        let token = nanoid::generate(config!(token_size) as usize);
        let insert_token = "INSERT INTO tokens(token, permissions, userid) VALUES ($1, $2, $3);";
        debug!(self.logger, "Creating Token"; "query" => insert_token);
        self.conn.execute(insert_token, &[&token, &permissions, &userid])?;
        Ok(token)
    }
}
