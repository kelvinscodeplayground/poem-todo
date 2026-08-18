use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Object)]
pub struct LoginRequestDto {
    pub username: String,
    pub password: String,
}
