use actix_session::Session;
use actix_web::{error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            id: self.id,
            name: self.name.clone(),
        }
    }
}

pub fn check_user(session: &Session) -> Result<User> {
    match session.get::<User>("user")? {
        Some(user) => Ok(user),
        None => Err(error::ErrorUnauthorized("Unauthorized")),
    }
}
