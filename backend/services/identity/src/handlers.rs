use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use crate::{routes::AppState, repositories::create_user, models::User};
use shared::errors::AppError;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String, // In reality, this should be a strong password
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub user: User,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    // Basic validation
    if payload.email.is_empty() || payload.password.is_empty() {
         return Err(AppError::BadRequest("Email and password are required".to_string()));
    }

    // Hash password (simplified for Phase 2)
    let password_hash = format!("hashed_{}", payload.password);

    let user = create_user(&state.db, &payload.email, &password_hash).await?;

    Ok(Json(RegisterResponse { user }))
}
