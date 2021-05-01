pub mod token;

use actix_session::Session;
use actix_web::{error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    id: i32,
}

impl Clone for LoginInfo {
    fn clone(&self) -> Self {
        LoginInfo { id: self.id }
    }
}

pub fn check_user(session: &Session) -> Result<LoginInfo> {
    match session.get::<LoginInfo>("user")? {
        Some(user) => Ok(user),
        None => Err(error::ErrorUnauthorized("Unauthorized")),
    }
}
