use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Product {
    pub id: Uuid,
    pub business_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub cost: Decimal,
    pub track_inventory: bool,
    pub current_stock: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct StockMovement {
    pub id: Uuid,
    pub business_id: Uuid,
    pub product_id: Uuid,
    pub movement_type: String, // IN, OUT, ADJUSTMENT
    pub quantity: Decimal,
    pub reference_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}
