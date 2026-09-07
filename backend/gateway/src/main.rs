use std::net::SocketAddr;
use reqwest::Client;
use tower_http::{
    trace::{TraceLayer, DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse},
    request_id::{SetRequestIdLayer, PropagateRequestIdLayer},
};
use shared::config::AppConfig;
use shared::logging::init_logging;
use shared::tracing::init_tracing;

mod routes;
mod middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    init_tracing("api_gateway");

    // In production, these should come from env config
    let config = AppConfig::from_env().unwrap_or(AppConfig {
         database_url: "".to_string(), // Gateway doesn't need DB usually
         server_port: 8000,
         nats_url: "localhost:4222".to_string(),
    });

    let identity_url = std::env::var("IDENTITY_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:8001".to_string());

    let state = routes::GatewayState {
        client: Client::new(),
        identity_url,
        business_url: std::env::var("BUSINESS_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8002".to_string()),
        accounting_url: std::env::var("ACCOUNTING_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8003".to_string()),
        transaction_url: std::env::var("TRANSACTION_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8004".to_string()),
        sales_url: std::env::var("SALES_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8005".to_string()),
        purchase_url: std::env::var("PURCHASE_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8006".to_string()),
        inventory_url: std::env::var("INVENTORY_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8007".to_string()),
        finance_url: std::env::var("FINANCE_SERVICE_URL").unwrap_or_else(|_| "http://localhost:8008".to_string()),
    };

    let app = routes::create_router(state)
        // Request ID middleware
        .layer(SetRequestIdLayer::x_request_id(middleware::tracing::MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        // Tracing middleware
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("API Gateway listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
