use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{Supplier, Bill};
use shared::errors::AppError;

pub async fn create_supplier(
    pool: &PgPool,
    business_id: Uuid,
    name: &str,
    email: Option<String>,
    phone: Option<String>,
) -> Result<Supplier, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO suppliers (id, business_id, name, email, phone)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
    "#;

    let supplier = sqlx::query_as::<_, Supplier>(query)
        .bind(id)
        .bind(business_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create supplier: {}", e);
            AppError::InternalError("Could not create supplier".to_string())
        })?;

    Ok(supplier)
}

pub async fn create_bill(
    pool: &PgPool,
    business_id: Uuid,
    supplier_id: Uuid,
    number: &str,
    date: chrono::NaiveDate,
    due_date: chrono::NaiveDate,
    total_amount: Decimal,
) -> Result<Bill, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO bills (id, business_id, supplier_id, number, date, due_date, status, total_amount)
        VALUES ($1, $2, $3, $4, $5, $6, 'DRAFT', $7)
        RETURNING *
    "#;

    let bill = sqlx::query_as::<_, Bill>(query)
        .bind(id)
        .bind(business_id)
        .bind(supplier_id)
        .bind(number)
        .bind(date)
        .bind(due_date)
        .bind(total_amount)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create bill: {}", e);
            AppError::InternalError("Could not create bill".to_string())
        })?;

    Ok(bill)
}
