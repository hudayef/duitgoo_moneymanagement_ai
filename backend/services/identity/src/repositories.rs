use sqlx::PgPool;
use uuid::Uuid;
use crate::models::User;
use shared::errors::AppError;

pub async fn create_user(pool: &PgPool, email: &str, password_hash: &str) -> Result<User, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO users (id, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING *
    "#;

    let user = sqlx::query_as::<_, User>(query)
        .bind(id)
        .bind(email)
        .bind(password_hash)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create user: {}", e);
            AppError::InternalError("Could not create user".to_string())
        })?;

    Ok(user)
}
