use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub business_id: Uuid,
    pub code: String,
    pub name: String,
    pub r#type: String, // ASSET, LIABILITY, EQUITY, REVENUE, EXPENSE
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Journal {
    pub id: Uuid,
    pub business_id: Uuid,
    pub date: NaiveDate,
    pub reference: Option<String>,
    pub description: Option<String>,
    pub idempotency_key: Option<String>,
    pub status: String, // POSTED, VOIDED
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct JournalLine {
    pub id: Uuid,
    pub journal_id: Uuid,
    pub account_id: Uuid,
    pub description: Option<String>,
    pub debit: Decimal,
    pub credit: Decimal,
    pub created_at: DateTime<Utc>,
}
