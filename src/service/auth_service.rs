use anyhow::Result;

use crate::{
    dto::register_request_dto::RegisterRequestDto,
    entity::user::User,
    repository::{base_repo::BaseRepo, sqlite_repo::SQLiteRepo},
    types::app_state::DbPool,
};

pub async fn create_user(db_pool: DbPool, new_user: &RegisterRequestDto) -> Result<()> {
    let repo = SQLiteRepo::new(db_pool);
    let user = User {
        id: uuid::Uuid::new_v4().to_string(),
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        password_hash: new_user.password.clone(), // In a real application, you should hash the password before storing it
        created_at: chrono::Utc::now().timestamp(),
    };
    repo.create_user(&user).await?;
    Ok(())
}
