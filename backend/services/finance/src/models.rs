use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct BankAccount {
    pub id: Uuid,
    pub business_id: Uuid,
    pub bank_name: String,
    pub account_number: String,
    pub account_name: String,
    pub currency: String,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Reconciliation {
    pub id: Uuid,
    pub business_id: Uuid,
    pub bank_account_id: Uuid,
    pub statement_date: NaiveDate,
    pub statement_balance: Decimal,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
