use postgres::{Client, Config, NoTls, Row};
use serde::Serialize;
use serde_json::{json, Value};

use crate::errors::UserError;
use crate::guards::Permission;
use crate::settings;
use crate::utils;

pub struct Database {
    conn: Client,
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub id: i32,
    pub token: String,
    pub permission: Permission,
    pub userid: i64,
    pub retired: bool,
}

#[derive(Debug, Serialize)]
pub struct Ban {
    pub id: i64,
    pub reason: String,
    pub date: chrono::NaiveDateTime,
    pub admin: i32,
}

impl Token {
    pub fn json(&self) -> Result<Value, UserError> {
        Ok(serde_json::to_value(&self)?)
    }
}

impl Ban {
    pub fn json(&self) -> Result<Value, UserError> {
        Ok(serde_json::to_value(self.raw_json())?)
    }

    pub fn raw_json(&self) -> Value {
        json!({
            "id": self.id,
            "reason": self.reason,
            "date": self.date.timestamp(),
            "admin": self.admin
        })
    }
}

impl Database {
    pub fn new() -> Result<Database, postgres::Error> {
        debug!(utils::LOGGER, "Connecting to database";
         "host" => &settings::ENV.database.host,
         "port" => settings::ENV.database.port,
         "name" => &settings::ENV.database.name,
         "username" => &settings::ENV.database.username);
        let conn = Config::new()
            .host(&settings::ENV.database.host)
            .port(settings::ENV.database.port)
            .dbname(&settings::ENV.database.name)
            .user(&settings::ENV.database.username)
            .password(&settings::ENV.database.password)
            .application_name(&env!("CARGO_PKG_NAME"))
            .connect(NoTls)?;
        debug!(utils::LOGGER, "Connected to PostgreSQL");
        Ok(Database { conn })
    }

    pub fn setup_tables(&mut self) -> Result<(), postgres::Error> {

        let permission_enum = "
            DO $$
            BEGIN
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'permission') THEN
                    CREATE TYPE permission AS ENUM ('User', 'Admin', 'Root');
                END IF;
            END$$;";
        debug!(utils::LOGGER, "Creating type `permission` if it doesn't exist";
            "query" => permission_enum, "name" => "banlist");
        self.conn.simple_query(permission_enum)?;

        let create_tokens = "
            CREATE TABLE IF NOT EXISTS tokens (
                id SERIAL PRIMARY KEY,
                token Text NOT NULL,
                permission permission NOT NULL,
                userid bigint NOT NULL,
                retired bool NOT NULL DEFAULT false);";

        debug!(utils::LOGGER, "Creating Table if it doesn't exist";
            "query" => create_tokens,  "name" => "tokens");
        self.conn.simple_query(create_tokens)?;

        let create_banlist = "
            CREATE TABLE IF NOT EXISTS banlist (
                id bigint NOT NULL PRIMARY KEY,
                reason Text NOT NULL,
                date timestamp NOT NULL,
                admin_token integer references tokens(id) NOT NULL);";
        debug!(utils::LOGGER, "Creating Table if it doesn't exist";
            "query" => create_banlist, "name" => "banlist");
        self.conn.simple_query(create_banlist)?;

        Ok(())
    }

    //region Tokens
    pub fn create_genesis_token(&mut self) -> Result<(), postgres::Error> {
        let get_genesis_token = "SELECT * FROM tokens WHERE id = 1;";
        debug!(utils::LOGGER, "Checking if Genesis Token exists";
            "query" => get_genesis_token);
        if self.conn.query(get_genesis_token, &[])?.is_empty() {
            info!(utils::LOGGER, "Genesis Token doesn't exist. Creating one";
                "size" => settings::ENV.general.token_size);
            let token = self.create_token(&Permission::Root, settings::ENV.general.masterid)?;
            info!(utils::LOGGER, "Created Genesis Token `{}`. Write this down, this will be the only time you see it.", token)
        } else {
            debug!(utils::LOGGER, "Genesis Token exists. Skipping creation.")
        }
        Ok(())
    }

    pub fn get_tokens(&mut self) -> Result<Vec<Token>, postgres::Error> {
        let get_all_tokens = "SELECT * FROM tokens;";
        debug!(utils::LOGGER, "Getting all tokens"; "query" => get_all_tokens);
        let result: Vec<Row> = self.conn.query(get_all_tokens, &[])?;
        Ok(result
            .into_iter()
            .map(|row| Token {
                id: row.get(0),
                token: row.get(1),
                permission: row.get(2),
                userid: row.get(3),
                retired: row.get(4),
            })
            .collect())
    }

    pub fn get_token_by_id(&mut self, token_id: i32) -> Result<Option<Token>, postgres::Error> {
        let get_token_by_id = "SELECT * FROM tokens WHERE id = $1;";
        debug!(utils::LOGGER, "Getting token by id";
            "id" => token_id, "query" => get_token_by_id);
        let row: Option<Row> = self.conn.query(get_token_by_id, &[&token_id])?.pop();

        Ok(match row {
            Some(token) => Some(Token {
                id: token.get(0),
                token: token.get(1),
                permission: token.get(2),
                userid: token.get(3),
                retired: token.get(4),
            }),
            None => None,
        })
    }

    pub fn get_token(&mut self, token: String) -> Result<Option<Token>, postgres::Error> {
        let get_token_by_id = "SELECT * FROM tokens WHERE token = $1;";
        debug!(utils::LOGGER, "Getting token"; "query" => get_token_by_id);
        let row: Option<Row> = self.conn.query(get_token_by_id, &[&token])?.pop();

        Ok(match row {
            Some(token) => Some(Token {
                id: token.get(0),
                token: token.get(1),
                permission: token.get(2),
                userid: token.get(3),
                retired: token.get(4),
            }),
            None => None,
        })
    }

    pub fn create_token(
        &mut self,
        permission: &Permission,
        userid: i64,
    ) -> Result<String, postgres::Error> {
        let token = nanoid::generate(settings::ENV.general.token_size as usize);
        let insert_token = "
            INSERT INTO tokens (
                token,
                permission,
                userid)
            VALUES ($1, $2, $3);";
        debug!(utils::LOGGER, "Creating Token";
         "query" => insert_token, "permission" => format!("{:?}", permission));
        self.conn.execute(insert_token, &[&token, &permission, &userid])?;
        Ok(token)
    }

    pub fn revoke_token_by_id(&mut self, token_id: i32) -> Result<(), postgres::Error> {
        let revoke_token_by_id = "UPDATE tokens SET retired = true WHERE id = $1;";
        debug!(utils::LOGGER, "Revoking token by id";
            "id" => token_id, "query" => revoke_token_by_id);
        self.conn.query(revoke_token_by_id, &[&token_id])?;
        Ok(())
    }
    //endregion

    //region Banlist
    pub fn get_bans(&mut self) -> Result<Vec<Ban>, postgres::Error> {
        let get_all_bans = "SELECT * FROM banlist;";
        debug!(utils::LOGGER, "Getting all bans"; "query" => get_all_bans);
        let result: Vec<Row> = self.conn.query(get_all_bans, &[])?;
        Ok(result
            .into_iter()
            .map(|row| Ban {
                id: row.get(0),
                reason: row.get(1),
                date: row.get(2),
                admin: row.get(3),
            })
            .collect())
    }

    pub fn add_ban(&mut self, user_id: i64, reason: &String, admin_token: i32) -> Result<(), postgres::Error> {
        let upsert_ban = "
            INSERT INTO banlist
            VALUES ($1, $2, now(), $3)
            ON CONFLICT (id) DO
            UPDATE SET reason=EXCLUDED.reason, date=excluded.date;";
        debug!(utils::LOGGER, "Upserting ban";
            "id" => &user_id, "reason" => &reason, "query" => upsert_ban);
        self.conn.query(upsert_ban, &[&user_id, &reason, &admin_token])?;
        Ok(())
    }

    pub fn get_ban(&mut self, user_id: i64) -> Result<Option<Ban>, postgres::Error> {
        let get_ban = "SELECT * FROM banlist WHERE id = $1;";
        debug!(utils::LOGGER, "Getting token by id";
            "id" => user_id, "query" => get_ban);
        let row: Option<Row> = self.conn.query(get_ban, &[&user_id])?.pop();

        Ok(match row {
            Some(ban) => Some(Ban {
                id: ban.get(0),
                reason: ban.get(1),
                date: ban.get(2),
                admin: ban.get(3),
            }),
            None => None,
        })
    }

    pub fn delete_ban(&mut self, user_id: i64) -> Result<(), postgres::Error> {
        let delete_ban = "DELETE FROM banlist WHERE id = $1;";
        debug!(utils::LOGGER, "Deleting ban";
            "id" => user_id, "query" => delete_ban);
        let row: Option<Row> = self.conn.query(delete_ban, &[&user_id])?.pop();

        Ok(())
    }
    //endregion
}
