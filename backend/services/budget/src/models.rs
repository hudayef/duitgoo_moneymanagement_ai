use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Budget {
    pub id: Uuid,
    pub business_id: Uuid,
    pub name: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: String, // DRAFT, ACTIVE, CLOSED
    pub total_budget: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct BudgetLine {
    pub id: Uuid,
    pub budget_id: Uuid,
    pub account_id: Uuid,
    pub allocated_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
