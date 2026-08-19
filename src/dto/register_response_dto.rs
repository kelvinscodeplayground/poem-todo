use poem_openapi::{ApiResponse, payload::PlainText};

use crate::service::auth_service::AuthServiceError;

/// Response type for the register endpoint
#[derive(ApiResponse)]
pub enum RegisterResponseType {
    /// User created successfully
    #[oai(status = 201)]
    Ok(PlainText<String>),
    /// User already exists
    #[oai(status = 409)]
    Conflict,
    /// Bad request
    #[oai(status = 400)]
    BadRequest(PlainText<String>),
    /// Internal server error
    #[oai(status = 500)]
    InternalServerError,
}

impl Default for RegisterResponseType {
    fn default() -> Self {
        RegisterResponseType::InternalServerError
    }
}

impl From<anyhow::Error> for RegisterResponseType {
    fn from(error: anyhow::Error) -> Self {
        match error.downcast_ref::<AuthServiceError>() {
            Some(AuthServiceError::UserAlreadyExists) => RegisterResponseType::Conflict,
            Some(AuthServiceError::PasswordRequirementsNotMet) => {
                RegisterResponseType::BadRequest(PlainText(error.to_string()))
            }
            Some(_) => {
                log::error!("Register Response Transform Error: {}", error);
                Default::default()
            }
            None => {
                log::error!("Register Response Transform Error: {}", error);
                Default::default()
            }
        }
    }
}
