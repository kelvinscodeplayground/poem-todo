use anyhow::{Context, Result};
use thiserror::Error;

use crate::{
    dto::register_request_dto::RegisterRequestDto, entity::user::User,
    repository::base::user_repository, types::app_state::DbPool,
};

/// Create a new user in the database
///
/// # Arguments
/// * `db_pool` - A reference to the database connection pool
/// * `new_user` - A reference to the RegisterRequestDto containing the new user's information
///
/// # Returns
/// * `Result<()>` - Returns Ok(()) if the user was created successfully
/// * `Err([`AuthServiceError::UserAlreadyExists`])` if the username already exists
///
/// # Errors
/// * `AuthServiceError::UserAlreadyExists` if the username already exists in the database
pub async fn create_user(db_pool: &DbPool, new_user: &RegisterRequestDto) -> Result<()> {
    let existing = get_user_by_username(db_pool, &new_user.username).await?;

    if existing.is_some() {
        return Err(AuthServiceError::UserAlreadyExists.into());
    }

    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        password_hash: hash_password(&new_user.password)?,
        created_at: chrono::Utc::now().timestamp(),
    };
    user_repository::create_user(&db_pool, &user).await?;
    Ok(())
}

fn hash_password(password: &str) -> Result<String> {
    use argon2::{
        Argon2, PasswordHasher,
        password_hash::{SaltString, rand_core::OsRng},
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .context("Failed to hash password")?;
    Ok(password_hash.to_string())
}

pub async fn get_user_by_username(db_pool: &DbPool, username: &str) -> Result<Option<User>> {
    let user = user_repository::get_user_by_username(&db_pool, username).await?;
    Ok(user)
}

// Custom error type for the auth service
#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("User already exists")]
    UserAlreadyExists,
}
