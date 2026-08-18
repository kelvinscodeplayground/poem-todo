pub type DbPool = sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub sql_pool: DbPool,
}
