use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{Customer, Invoice};
use shared::errors::AppError;

pub async fn create_customer(
    pool: &PgPool,
    business_id: Uuid,
    name: &str,
    email: Option<String>,
    phone: Option<String>,
) -> Result<Customer, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO customers (id, business_id, name, email, phone)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
    "#;

    let customer = sqlx::query_as::<_, Customer>(query)
        .bind(id)
        .bind(business_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create customer: {}", e);
            AppError::InternalError("Could not create customer".to_string())
        })?;

    Ok(customer)
}

pub async fn create_invoice(
    pool: &PgPool,
    business_id: Uuid,
    customer_id: Uuid,
    number: &str,
    date: chrono::NaiveDate,
    due_date: chrono::NaiveDate,
    total_amount: Decimal,
) -> Result<Invoice, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO invoices (id, business_id, customer_id, number, date, due_date, status, total_amount)
        VALUES ($1, $2, $3, $4, $5, $6, 'DRAFT', $7)
        RETURNING *
    "#;

    let invoice = sqlx::query_as::<_, Invoice>(query)
        .bind(id)
        .bind(business_id)
        .bind(customer_id)
        .bind(number)
        .bind(date)
        .bind(due_date)
        .bind(total_amount)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create invoice: {}", e);
            AppError::InternalError("Could not create invoice".to_string())
        })?;

    Ok(invoice)
}
