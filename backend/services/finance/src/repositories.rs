use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{BankAccount, Reconciliation};
use shared::errors::AppError;

pub async fn create_bank_account(
    pool: &PgPool,
    business_id: Uuid,
    bank_name: &str,
    account_number: &str,
    account_name: &str,
    currency: &str,
) -> Result<BankAccount, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO bank_accounts (id, business_id, bank_name, account_number, account_name, currency)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
    "#;

    let account = sqlx::query_as::<_, BankAccount>(query)
        .bind(id)
        .bind(business_id)
        .bind(bank_name)
        .bind(account_number)
        .bind(account_name)
        .bind(currency)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create bank account: {}", e);
            AppError::InternalError("Could not create bank account".to_string())
        })?;

    Ok(account)
}

pub async fn start_reconciliation(
    pool: &PgPool,
    business_id: Uuid,
    bank_account_id: Uuid,
    statement_date: chrono::NaiveDate,
    statement_balance: Decimal,
    notes: Option<String>,
) -> Result<Reconciliation, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO reconciliations (id, business_id, bank_account_id, statement_date, statement_balance, status, notes)
        VALUES ($1, $2, $3, $4, $5, 'DRAFT', $6)
        RETURNING *
    "#;

    let rec = sqlx::query_as::<_, Reconciliation>(query)
        .bind(id)
        .bind(business_id)
        .bind(bank_account_id)
        .bind(statement_date)
        .bind(statement_balance)
        .bind(notes)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to start reconciliation: {}", e);
            AppError::InternalError("Could not start reconciliation".to_string())
        })?;

    Ok(rec)
}
