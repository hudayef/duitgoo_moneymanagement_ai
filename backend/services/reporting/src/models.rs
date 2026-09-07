use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct FinancialReport {
    pub id: Uuid,
    pub business_id: Uuid,
    pub report_type: String, // TRIAL_BALANCE, PROFIT_LOSS, BALANCE_SHEET
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub report_data: sqlx::types::Json<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}
