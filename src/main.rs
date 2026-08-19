use std::str::FromStr;

use anyhow::Result;
use log;
use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::Cors};
use poem_openapi::OpenApiService;

use crate::{
    controller::{auth_controller::AuthController, todo_controller::TodoController},
    types::app_state::DbPool,
};

mod controller;
mod dto;
mod entity;
mod repository;
mod service;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "poem=debug,poem-todo=debug");
        }
    }

    tracing_subscriber::fmt::init();
    log::info!("Initializing app...");
    let state = create_initial_state().await?;
    migrate_db(&state.sql_pool).await?;

    let api_service = OpenApiService::new((AuthController, TodoController), "Hello world", "1.0")
        .server("http://localhost:8080");
    let ui = api_service.swagger_ui();
    let app = Route::new()
        .nest("/", api_service)
        .nest("/docs", ui)
        .with(Cors::new())
        .data(state);
    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(app)
        .await?;

    Ok(())
}

async fn migrate_db(pool: &DbPool) -> Result<()> {
    log::info!("Migrating database...");
    match pool {
        DbPool::Sqlite(pool) => sqlx::migrate!("./migrations").run(pool).await?,
    }
    Ok(())
}

async fn create_initial_state() -> Result<types::app_state::AppState> {
    use sqlx::sqlite::SqliteConnectOptions;
    use sqlx::sqlite::SqlitePool;

    log::info!("Creating initial state...");
    let options = SqliteConnectOptions::from_str("sqlite:./todo.db")?.create_if_missing(true);
    let pool = DbPool::Sqlite(SqlitePool::connect_with(options).await?);
    Ok(types::app_state::AppState { sql_pool: pool })
}
