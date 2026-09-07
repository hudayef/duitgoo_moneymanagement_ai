use sqlx::PgPool;
use uuid::Uuid;
use crate::models::Transaction;
use shared::errors::AppError;

pub async fn create_transaction(
    pool: &PgPool,
    business_id: Uuid,
    txn_type: &str,
    idempotency_key: Option<String>,
    reference: Option<String>,
) -> Result<Transaction, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO transactions (id, business_id, transaction_type, status, idempotency_key, reference)
        VALUES ($1, $2, $3, 'PENDING', $4, $5)
        RETURNING *
    "#;

    let transaction = sqlx::query_as::<_, Transaction>(query)
        .bind(id)
        .bind(business_id)
        .bind(txn_type)
        .bind(idempotency_key)
        .bind(reference)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create transaction: {}", e);
            AppError::InternalError("Could not create transaction".to_string())
        })?;

    Ok(transaction)
}

pub async fn update_transaction_status(
    pool: &PgPool,
    id: Uuid,
    status: &str,
) -> Result<Transaction, AppError> {
    let query = r#"
        UPDATE transactions SET status = $1, updated_at = NOW() WHERE id = $2
        RETURNING *
    "#;

    let transaction = sqlx::query_as::<_, Transaction>(query)
        .bind(status)
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update transaction status: {}", e);
            AppError::InternalError("Could not update transaction".to_string())
        })?;

    Ok(transaction)
}
