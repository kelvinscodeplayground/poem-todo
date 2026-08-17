use anyhow::Result;
use log;
use poem::{Route, Server, listener::TcpListener};
use poem_openapi::{OpenApi, OpenApiService, param::Query, payload::PlainText};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(n) => PlainText(format!("Hello! {}", n)),
            None => PlainText("Oh Hi!".to_string()),
        }
    }
}

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
        OpenApiService::new(Api, "Hello world", "1.0").server("http://localhost:8080");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);
    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(app)
        .await?;

    Ok(())
}
