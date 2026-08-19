use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use poem::web::{Data, Json};
use poem::{Error, Result, http::StatusCode};
use poem_openapi::{OpenApi, payload::PlainText};

use crate::{
    dto::{login_request_dto::LoginRequestDto, register_request_dto::RegisterRequestDto},
    service::auth_service::{self, AuthSerivceError},
    types::app_state::AppState,
};

pub struct AuthController;

#[OpenApi]
impl AuthController {
    #[oai(path = "/login", method = "post")]
    pub async fn login(&self, credential: Json<LoginRequestDto>) -> PlainText<String> {
        let password = credential.password.as_bytes();
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2.hash_password(password, &salt);
        match hash {
            Ok(hash) => {
                let hash_str = hash.to_string();
                let read_hash = argon2::PasswordHash::new(&hash_str);

                let verified = argon2.verify_password(password, &read_hash.unwrap());
                match verified {
                    Ok(_) => log::info!("Password verified successfully"),
                    Err(e) => log::error!("Failed to verify password: {}", e),
                }
                PlainText(hash_str)
            }
            Err(e) => {
                log::error!("Failed to hash password: {}", e);
                PlainText("Error".into())
            }
        }
    }

    /// Register a new user
    ///
    /// Allow users to register by providing a username, password, and email. The password will be hashed using Argon2
    /// before storing it in the database.
    #[oai(path = "/register", method = "post")]
    pub async fn register(
        &self,
        data: Data<&AppState>,
        body: Json<RegisterRequestDto>,
    ) -> Result<PlainText<String>> {
        let result = auth_service::create_user(&data.sql_pool, &body).await;
        if let Err(e) = &result {
            log::error!("Failed to create user: {}", e);
            match e.downcast_ref() {
                Some(AuthSerivceError::UserAlreadyExists) => {
                    return Err(Error::from_status(StatusCode::CONFLICT));
                }
                _ => {
                    return Err(Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
                }
            }
        };

        Ok(PlainText("".to_string()))
    }
}
