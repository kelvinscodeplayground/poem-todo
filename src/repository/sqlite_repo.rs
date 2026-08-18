use crate::repository::base_repo;
use anyhow::Result;

pub struct SQLiteRepo {
    pool: sqlx::SqlitePool,
}

impl SQLiteRepo {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

impl base_repo::BaseRepo for SQLiteRepo {
    async fn create_user(&self, user: &crate::entity::user::User) -> Result<()> {
        sqlx::query(
                "INSERT INTO users (id, username, email, password_hash, created_at) VALUES (?, ?, ?, ?, ?)"
            ).bind(&user.id)
             .bind(&user.username)
             .bind(&user.email)
             .bind(&user.password_hash)
             .bind(user.created_at).execute(&self.pool).await?;

        Ok(())
    }
}
