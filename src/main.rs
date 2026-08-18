use anyhow::Result;
use log;
use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;

use crate::controller::todo_controller::TodoController;

mod controller;
mod dto;
mod entity;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "poem=debug,poem-todo=debug");
        }
    }

    tracing_subscriber::fmt::init();
    log::info!("Initializing app...");

    let api_service =
        OpenApiService::new(TodoController, "Hello world", "1.0").server("http://localhost:8080");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);
    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(app)
        .await?;

    Ok(())
}
