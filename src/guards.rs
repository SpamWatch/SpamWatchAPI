use chrono::{Duration, FixedOffset, NaiveDateTime, NaiveTime, Timelike, Utc};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

use crate::database::{Antiflood, AntifloodColumn, Database};
use crate::database::Token;
use crate::errors::UserError;

#[derive(Debug, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "permission")]
pub enum Permission {
    // Can read from the API
    User,
    // Can add IDs to the API
    Admin,
    // Can create/revoke tokens
    Root,
}


pub struct TokenGuard {
    pub token: Token,
    db: Database,
    antiflood: Antiflood,
}

impl TokenGuard {
    pub fn new(token_header: String) -> Result<TokenGuard, UserError> {
        let mut db = Database::new()?;
        if !token_header.is_empty() {
            let token = match db.get_token(token_header)? {
                Some(token) => token,
                None => return Err(UserError::Unauthorized),
            };
            let antiflood = db.get_antiflood(token.id)?;

            if token.retired {
                return Err(UserError::Unauthorized);
            }

            Ok(TokenGuard { token, db, antiflood })
        } else {
            return Err(UserError::Unauthorized);
        }
    }

    pub fn admin(&self) -> bool {
        match self.token.permission {
            Permission::Admin => true,
            Permission::Root => true,
            _ => false,
        }
    }

    pub fn root(&self) -> bool {
        match self.token.permission {
            Permission::Root => true,
            _ => false,
        }
    }

    pub fn banlist_all(&mut self) -> Result<(), UserError> {
        if self.admin() {
            return Ok(())
        }
        let current_time = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
        if self.antiflood.banlist_all < current_time {
            self.db.set_antiflood_banlist_all(self.token.id,
                                              current_time + Duration::minutes(5))?;
            Ok(())
        } else {
            return Err(UserError::TooManyRequests { until: self.antiflood.banlist_all.timestamp() });
        }
    }
}
