use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct RegisterRequestDto {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub email: String,
}
