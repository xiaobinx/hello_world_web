pub mod app_state;
pub mod token;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenInfo {
    id: i32,
    name: String,
}
