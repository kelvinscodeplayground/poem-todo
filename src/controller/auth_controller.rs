use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use poem::{Result, http::StatusCode};
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::{
    dto::{login_request_dto::LoginRequestDto, register_request_dto::RegisterRequestDto},
    entity::user::User,
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
    pub async fn register(&self, body: Json<RegisterRequestDto>) -> Result<Json<User>> {
        let password = body.password.as_bytes();
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2.hash_password(password, &salt);
        match hash {
            Ok(hash) => {
                let hash_str = hash.to_string();
                log::info!("Password hashed successfully: {}", hash_str);
                // Here you would typically save the user to the database
                // For demonstration, we will just return a dummy user
                let user = User {
                    id: "dummy_id".into(),
                    username: body.username.clone(),
                    email: body.email.clone(),
                    password_hash: hash_str,
                    created_at: chrono::Utc::now().timestamp(),
                };
                Ok(Json(user))
            }
            Err(e) => {
                log::error!("Failed to hash password: {}", e);
                Err(poem::Error::from_string(
                    "Failed to register user",
                    poem::http::StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
    }
}
