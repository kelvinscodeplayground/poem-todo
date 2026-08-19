use poem_openapi::{
    ApiResponse, Object,
    payload::{Json, PlainText},
};
use serde::{Deserialize, Serialize};

use crate::service::auth_service::AuthServiceError;

/// Response DTO for the login endpoint
#[derive(Debug, Object, Serialize, Deserialize, Default)]
pub struct LoginResponseDto {
    /// JWT token for the authenticated user
    pub token: String,
    /// Username of the authenticated user
    pub username: String,
    /// Expiration time of the JWT token in Unix timestamp format
    pub expires_at: i64,
}

#[derive(Debug, ApiResponse)]
pub enum LoginResponseType {
    #[oai(status = 200)]
    Ok(Json<LoginResponseDto>),
    #[oai(status = 401)]
    Unauthorized(PlainText<String>),
    #[oai(status = 500)]
    InternalServerError,
}

impl Default for LoginResponseType {
    fn default() -> Self {
        LoginResponseType::InternalServerError
    }
}

impl From<anyhow::Error> for LoginResponseType {
    fn from(error: anyhow::Error) -> Self {
        match error.downcast_ref::<AuthServiceError>() {
            Some(AuthServiceError::BadCredentials) => {
                LoginResponseType::Unauthorized(PlainText(error.to_string()))
            }
            Some(AuthServiceError::UserNotFound) => {
                LoginResponseType::Unauthorized(PlainText(error.to_string()))
            }
            Some(_) => {
                log::error!("Login Response Transform Error: {}", error);
                Default::default()
            }
            None => {
                log::error!("Login Response Transform Error: {}", error);
                Default::default()
            }
        }
    }
}
