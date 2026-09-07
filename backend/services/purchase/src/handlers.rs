use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use crate::{routes::AppState, repositories::*, models::*};
use shared::errors::AppError;

#[derive(Deserialize)]
pub struct CreateSupplierRequest {
    pub business_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct SupplierResponse {
    pub supplier: Supplier,
}

pub async fn handle_create_supplier(
    State(state): State<AppState>,
    Json(payload): Json<CreateSupplierRequest>,
) -> Result<Json<SupplierResponse>, AppError> {
    let supplier = create_supplier(&state.db, payload.business_id, &payload.name, payload.email, payload.phone).await?;
    Ok(Json(SupplierResponse { supplier }))
}

#[derive(Deserialize)]
pub struct CreateBillRequest {
    pub business_id: Uuid,
    pub supplier_id: Uuid,
    pub number: String,
    pub date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_amount: Decimal,
}

#[derive(Serialize)]
pub struct BillResponse {
    pub bill: Bill,
}

pub async fn handle_create_bill(
    State(state): State<AppState>,
    Json(payload): Json<CreateBillRequest>,
) -> Result<Json<BillResponse>, AppError> {
    let bill = create_bill(
        &state.db,
        payload.business_id,
        payload.supplier_id,
        &payload.number,
        payload.date,
        payload.due_date,
        payload.total_amount,
    ).await?;

    Ok(Json(BillResponse { bill }))
}
