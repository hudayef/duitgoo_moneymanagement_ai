use sqlx::PgPool;
use uuid::Uuid;
use crate::models::Business;
use shared::errors::AppError;

pub async fn create_business(pool: &PgPool, owner_id: Uuid, name: &str) -> Result<Business, AppError> {
    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {}", e);
        AppError::InternalError("Database error".to_string())
    })?;

    let business_id = Uuid::new_v4();
    let query_biz = r#"
        INSERT INTO businesses (id, name)
        VALUES ($1, $2)
        RETURNING *
    "#;

    let business = sqlx::query_as::<_, Business>(query_biz)
        .bind(business_id)
        .bind(name)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create business: {}", e);
            AppError::InternalError("Could not create business".to_string())
        })?;

    let query_mem = r#"
        INSERT INTO memberships (user_id, business_id, role)
        VALUES ($1, $2, 'OWNER')
    "#;

    sqlx::query(query_mem)
        .bind(owner_id)
        .bind(business_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create membership: {}", e);
            AppError::InternalError("Could not assign business owner".to_string())
        })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit transaction: {}", e);
        AppError::InternalError("Database error".to_string())
    })?;

    Ok(business)
}

pub async fn get_user_businesses(pool: &PgPool, user_id: Uuid) -> Result<Vec<Business>, AppError> {
    let query = r#"
        SELECT b.* FROM businesses b
        JOIN memberships m ON b.id = m.business_id
        WHERE m.user_id = $1
    "#;

    let businesses = sqlx::query_as::<_, Business>(query)
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch businesses: {}", e);
            AppError::InternalError("Could not fetch businesses".to_string())
        })?;

    Ok(businesses)
}
