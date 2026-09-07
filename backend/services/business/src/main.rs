use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use shared::config::AppConfig;
use shared::logging::init_logging;
use shared::tracing::init_tracing;

mod routes;
mod models;
mod repositories;
mod handlers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    init_tracing("business_service");

    let config = AppConfig::from_env().unwrap_or(AppConfig {
         database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://duitgoo_user:duitgoo_password@localhost:5432/duitgoo_db".to_string()),
         server_port: 8002,
         nats_url: "localhost:4222".to_string(),
    });

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let state = routes::AppState {
        db: db_pool,
    };

    let app = routes::create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("Business service listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
