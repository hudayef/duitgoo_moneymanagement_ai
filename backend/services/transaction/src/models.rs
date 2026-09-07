use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub business_id: Uuid,
    pub transaction_type: String, // SALE, PURCHASE, TRANSFER
    pub status: String, // PENDING, COMPLETED, FAILED, VOIDED
    pub idempotency_key: Option<String>,
    pub reference: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
