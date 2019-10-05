use postgres_derive::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::database::Token;
use crate::errors::UserError;

#[derive(Debug, FromSql, ToSql, Serialize, Deserialize)]
#[postgres(name = "permission")]
pub enum Permission {
    // Can read from the API
    User,
    // Can add IDs to the API
    Admin,
    // Can create/revoke tokens
    Root,
}

#[derive(Debug)]
pub struct PermissionGuard {
    token: Token,
}

impl PermissionGuard {
    pub fn new(token_header: String) -> Result<PermissionGuard, UserError> {
        let mut db = Database::new()?;
        if !token_header.is_empty() {
            let token = match db.get_token(token_header)? {
                Some(token) => token,
                None => return Err(UserError::Unauthorized)
            };

            Ok(PermissionGuard {
                token,
            })
        } else {
            return Err(UserError::Unauthorized);
        }
    }

    pub fn admin(&self) -> bool {
        match self.token.permissions {
            Permission::Admin => true,
            Permission::Root => true,
            _ => false
        }
    }

    pub fn root(&self) -> bool {
        match self.token.permissions {
            Permission::Root => true,
            _ => false
        }
    }
}
