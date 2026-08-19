#[derive(Clone)]
pub struct AppState {
    pub sql_pool: DbPool,
}

#[derive(Clone, Debug)]
pub enum DbPool {
    Sqlite(sqlx::SqlitePool),
}
