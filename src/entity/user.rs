use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, Object, Default, FromRow)]
pub struct User {
    /// uuid of the user
    pub id: String,
    /// username of the user
    pub username: String,
    /// email of the user
    pub email: String,
    /// hashed password of the user
    pub password_hash: String,
    /// timestamp of when the user was created
    pub created_at: i64,
}
