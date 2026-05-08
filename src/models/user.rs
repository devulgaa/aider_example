use serde::{Deserialize, Serialize};

/// Represents a user.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub birthdate: Option<String>,
}
