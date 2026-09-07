use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{Budget, BudgetLine};
use shared::errors::AppError;

pub async fn create_budget(
    pool: &PgPool,
    business_id: Uuid,
    name: &str,
    period_start: chrono::NaiveDate,
    period_end: chrono::NaiveDate,
    total_budget: Decimal,
) -> Result<Budget, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO budgets (id, business_id, name, period_start, period_end, status, total_budget)
        VALUES ($1, $2, $3, $4, $5, 'DRAFT', $6)
        RETURNING *
    "#;

    let budget = sqlx::query_as::<_, Budget>(query)
        .bind(id)
        .bind(business_id)
        .bind(name)
        .bind(period_start)
        .bind(period_end)
        .bind(total_budget)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create budget: {}", e);
            AppError::InternalError("Could not create budget".to_string())
        })?;

    Ok(budget)
}

pub async fn add_budget_line(
    pool: &PgPool,
    budget_id: Uuid,
    account_id: Uuid,
    allocated_amount: Decimal,
) -> Result<BudgetLine, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO budget_lines (id, budget_id, account_id, allocated_amount)
        VALUES ($1, $2, $3, $4)
        RETURNING *
    "#;

    let line = sqlx::query_as::<_, BudgetLine>(query)
        .bind(id)
        .bind(budget_id)
        .bind(account_id)
        .bind(allocated_amount)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to add budget line: {}", e);
            AppError::InternalError("Could not add budget line".to_string())
        })?;

    Ok(line)
}
