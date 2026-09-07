use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::{routes::AppState, repositories::*, models::*};
use shared::errors::AppError;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub business_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub cost: Decimal,
    pub track_inventory: bool,
}

#[derive(Serialize)]
pub struct ProductResponse {
    pub product: Product,
}

pub async fn handle_create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<Json<ProductResponse>, AppError> {
    let product = create_product(
        &state.db,
        payload.business_id,
        &payload.sku,
        &payload.name,
        payload.description,
        payload.price,
        payload.cost,
        payload.track_inventory
    ).await?;

    Ok(Json(ProductResponse { product }))
}

#[derive(Deserialize)]
pub struct AdjustStockRequest {
    pub business_id: Uuid,
    pub product_id: Uuid,
    pub movement_type: String,
    pub quantity: Decimal,
    pub reference_id: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct StockMovementResponse {
    pub movement: StockMovement,
}

pub async fn handle_adjust_stock(
    State(state): State<AppState>,
    Json(payload): Json<AdjustStockRequest>,
) -> Result<Json<StockMovementResponse>, AppError> {
    let movement = adjust_stock(
        &state.db,
        payload.business_id,
        payload.product_id,
        &payload.movement_type,
        payload.quantity,
        payload.reference_id,
        payload.notes,
    ).await?;

    Ok(Json(StockMovementResponse { movement }))
}
