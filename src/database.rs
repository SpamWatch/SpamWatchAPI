use chrono::NaiveDateTime;
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
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Antiflood {
    pub banlist_all: NaiveDateTime,
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
            "admin": self.admin,
            "message": self.message
        })
    }
}

impl Default for Antiflood {
    fn default() -> Self {
        Antiflood {
            banlist_all: NaiveDateTime::from_timestamp(0, 0)
        }
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

    pub fn get_token_by_userid(&mut self, userid: i64) -> Result<Vec<Token>, postgres::Error> {
        let get_token_by_id = "SELECT * FROM tokens WHERE userid = $1;";
        debug!(utils::LOGGER, "Getting token by userid";
            "id" => userid, "query" => get_token_by_id);
        let result: Vec<Row> = self.conn.query(get_token_by_id, &[&userid])?;

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
                message: row.try_get(4).unwrap_or(Some("test".to_string())),
            })
            .collect())
    }

    pub fn get_banned_ids(&mut self) -> Result<Vec<i64>, postgres::Error> {
        let get_all_bans = "SELECT id FROM banlist;";
        debug!(utils::LOGGER, "Getting all bans as ids"; "query" => get_all_bans);
        let result: Vec<Row> = self.conn.query(get_all_bans, &[])?;
        Ok(result
            .into_iter()
            .map(|row| row.get(0))
            .collect())
    }

    pub fn get_total_ban_count(&mut self) -> Result<i64, postgres::Error> {
        let get_all_bans = "SELECT COUNT(*) FROM banlist;";
        debug!(utils::LOGGER, "Getting all bans"; "query" => get_all_bans);
        let result: Vec<Row> = self.conn.query(get_all_bans, &[])?;
        let count = match result.get(0) {
            Some(row) => row.get(0),
            None => 0
        };
        Ok(count)
    }

    pub fn add_ban(&mut self, user_id: i64, reason: &String, admin_token: i32, message: &Option<String>) -> Result<(), postgres::Error> {
        let upsert_ban = "
            INSERT INTO banlist
            VALUES ($1, $2, now(), $3, $4)
            ON CONFLICT (id) DO
            UPDATE SET reason=excluded.reason, date=excluded.date, message=excluded.message;";
        debug!(utils::LOGGER, "Upserting ban";
            "id" => &user_id, "reason" => &reason, "query" => upsert_ban);
        self.conn.query(upsert_ban, &[&user_id, &reason, &admin_token, &message])?;
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
                message: ban.try_get(4).unwrap_or(None),
            }),
            None => None,
        })
    }

    pub fn delete_ban(&mut self, user_id: i64) -> Result<(), postgres::Error> {
        let delete_ban = "DELETE FROM banlist WHERE id = $1;";
        debug!(utils::LOGGER, "Deleting ban";
            "id" => user_id, "query" => delete_ban);
        self.conn.query(delete_ban, &[&user_id])?.pop();

        Ok(())
    }
    //endregion

    //region Antiflood
    pub fn get_antiflood(&mut self, token_id: i32) -> Result<Antiflood, postgres::Error> {
        let get_ban = "SELECT (banlist_all) FROM antiflood WHERE token = $1;";
        debug!(utils::LOGGER, "Getting token antiflood settings";
            "token" => token_id, "query" => get_ban);
        let row: Option<Row> = self.conn.query(get_ban, &[&token_id])?.pop();

        Ok(match row {
            Some(antiflood) => Antiflood {
                banlist_all: antiflood.get(0),
            },
            None => Antiflood::default(),
        })
    }

    pub fn set_antiflood_banlist_all(&mut self, token_id: i32, time: NaiveDateTime) -> Result<(), postgres::Error> {
        let upsert_antiflood = "
            INSERT INTO antiflood (token, banlist_all)
            VALUES ($1, $2)
            ON CONFLICT (token) DO
            UPDATE SET banlist_all=EXCLUDED.banlist_all;";
        debug!(utils::LOGGER, "Updating antiflood";
            "token" => &token_id, "column" => "banlist_all", "query" => upsert_antiflood);
        self.conn.query(upsert_antiflood, &[&token_id, &time])?;
        Ok(())
    }
    //endregion
}
