use postgres::{Client, Config, NoTls};

use crate::settings::Settings;
use crate::utils;

pub struct Database {
    conn: Client,
}

impl Database {
    pub fn new() -> Database {
        let logger = utils::logger();
        let settings = Settings::load();

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
            conn
        }
    }

    pub fn setup_tables(&mut self) {
        let logger = utils::logger();
        let create_banlist = "CREATE TABLE IF NOT EXISTS banlist (id integer NOT NULL PRIMARY KEY, reason Text NOT NULL, date timestamp NOT NULL);";
        debug!(logger, "Creating Table"; "query" => create_banlist, "name" => "banlist");
        self.conn.simple_query(create_banlist).unwrap();

        let create_tokens = "CREATE TABLE IF NOT EXISTS tokens (id SERIAL, token Text NOT NULL PRIMARY KEY, permissions json NOT NULL, userid integer NOT NULL);";
        debug!(logger, "Creating Table";"query" => create_tokens,  "name" => "tokens");
        self.conn.simple_query(create_tokens).unwrap();
    }

    pub fn create_genesis_token(&mut self) {
        let logger = utils::logger();
        let settings = Settings::load();
        let get_genesis_token = "SELECT * FROM tokens WHERE id = 1;";
        debug!(logger, "Checking if Genesis token exists"; "query" => get_genesis_token);
        if self.conn.query(get_genesis_token, &[]).unwrap().is_empty() {
            info!(logger, "Genesis Token doesn't exist. Creating one"; "size" => settings.token_size);
            let token = nanoid::generate(settings.token_size as usize);
            let insert_token = r#"INSERT INTO tokens(token, permissions, userid) VALUES ($1, '{"all": "rw"}', $2);"#;
            debug!(logger, "Creating Token"; "query" => insert_token);
            self.conn.execute(insert_token, &[&token, &settings.masterid]).unwrap();
        }
        let token: String = self.conn.query(get_genesis_token, &[]).unwrap()[0].get("token");
        info!(logger, "Found Genesis Token"; "token" => token);
    }
}
