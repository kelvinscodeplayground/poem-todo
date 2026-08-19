use anyhow::Result;
use sqlx::SqlitePool;

pub async fn create_user(pool: &SqlitePool, user: &crate::entity::user::User) -> Result<()> {
    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&user.id)
    .bind(&user.username)
    .bind(&user.email)
    .bind(&user.password_hash)
    .bind(user.created_at)
    .execute(pool)
    .await?;

    Ok(())
}
