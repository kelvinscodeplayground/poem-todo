use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::dto::login_request_dto::LoginRequestDto;

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
}
