use axum::{
    routing::{get, post},
    Router,
    Json,
};
use sqlx::PgPool;
use serde::Serialize;
use shared::errors::AppError;
use crate::handlers::{handle_create_bank_account, handle_start_reconciliation};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/metrics", get(metrics))
        .route("/version", get(version))
        .route("/bank-accounts", post(handle_create_bank_account))
        .route("/reconciliations", post(handle_start_reconciliation))
        .with_state(state)
}

async fn healthz() -> Result<Json<HealthResponse>, AppError> {
    Ok(Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    }))
}

async fn readyz(axum::extract::State(state): axum::extract::State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    if let Err(_) = sqlx::query("SELECT 1").execute(&state.db).await {
         return Err(AppError::InternalError("Database not ready".into()));
    }

    Ok(Json(HealthResponse {
        status: "ready".to_string(),
        version: "0.1.0".to_string(),
    }))
}

async fn metrics() -> Result<String, AppError> {
    Ok("# HELP http_requests_total The total number of HTTP requests.\n# TYPE http_requests_total counter\nhttp_requests_total 0".to_string())
}

async fn version() -> Result<Json<HealthResponse>, AppError> {
     Ok(Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    }))
}
