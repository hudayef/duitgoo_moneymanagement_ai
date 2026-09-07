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
    use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Argon2};
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt).map_err(|_| AppError::InternalError("Hashing failed".into()))?.to_string();

    let user = create_user(&state.db, &payload.email, &password_hash).await?;

    Ok(Json(RegisterResponse { user }))
}
