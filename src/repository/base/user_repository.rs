use crate::{entity::user::User, repository::sqlite, types::app_state::DbPool};
use anyhow::Result;

pub async fn create_user(pool: &DbPool, user: &User) -> Result<()> {
    match pool {
        DbPool::Sqlite(pool) => sqlite::user_repository::create_user(pool, user).await,
    }
}
