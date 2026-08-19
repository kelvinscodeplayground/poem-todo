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
///
/// # Errors
/// * `AuthServiceError::UserAlreadyExists` if the username already exists in the database
/// * `AuthServiceError::PasswordRequirementsNotMet` if the password does not meet the requirements
pub async fn create_user(db_pool: &DbPool, new_user: &RegisterRequestDto) -> Result<()> {
    let existing = get_user_by_username(db_pool, &new_user.username).await?;

    if existing.is_some() {
        return Err(AuthServiceError::UserAlreadyExists.into());
    }

    let pwd = &new_user.password;
    let pwd_valid = new_user.password == new_user.confirm_password
        && pwd.len() >= 8
        && pwd.chars().any(|c| c.is_lowercase())
        && pwd.chars().any(|c| c.is_uppercase())
        && pwd.chars().any(|c| c.is_numeric())
        && pwd
            .chars()
            .any(|c| matches!(c, '@' | '$' | '!' | '%' | '*' | '?' | '&'));

    if !pwd_valid {
        return Err(AuthServiceError::PasswordRequirementsNotMet.into());
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

/// Authenticate a user by verifying their username and password
///
/// # Return
/// * `Result<User>` - Returns the authenticated user if the credentials are valid
///
/// # Errors
/// * `AuthServiceError::BadCredentials` if the username or password is incorrect
/// * `AuthServiceError::UserNotFound` if the user does not exist in the database
pub async fn login_user(db_pool: &DbPool, username: &str, password: &str) -> Result<User> {
    let user = match get_user_by_username(db_pool, username).await? {
        Some(user) => user,
        None => {
            return Err(AuthServiceError::UserNotFound.into());
        }
    };

    let verified = verify_password(password, &user.password_hash).await;
    if !verified {
        return Err(AuthServiceError::BadCredentials.into());
    }

    Ok(user)
}

pub async fn get_user_by_username(db_pool: &DbPool, username: &str) -> Result<Option<User>> {
    let user = user_repository::get_user_by_username(&db_pool, username).await?;
    Ok(user)
}

pub async fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{Argon2, PasswordVerifier, password_hash::PasswordHash};

    let parsed_hash = match PasswordHash::new(hash) {
        Ok(hash) => hash,
        Err(e) => {
            log::error!("Error parsing password hash: {}", e);
            return false;
        }
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

// Private functions.
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

// Custom error type for the auth service
#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Password does not meet the requirements")]
    PasswordRequirementsNotMet,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid username or password")]
    BadCredentials,
}
