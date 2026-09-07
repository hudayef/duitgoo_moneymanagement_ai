use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use crate::{routes::AppState, repositories::*, models::*};
use shared::errors::AppError;

#[derive(Deserialize)]
pub struct CreateCustomerRequest {
    pub business_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct CustomerResponse {
    pub customer: Customer,
}

pub async fn handle_create_customer(
    State(state): State<AppState>,
    Json(payload): Json<CreateCustomerRequest>,
) -> Result<Json<CustomerResponse>, AppError> {
    let customer = create_customer(&state.db, payload.business_id, &payload.name, payload.email, payload.phone).await?;
    Ok(Json(CustomerResponse { customer }))
}

#[derive(Deserialize)]
pub struct CreateInvoiceRequest {
    pub business_id: Uuid,
    pub customer_id: Uuid,
    pub number: String,
    pub date: NaiveDate,
    pub due_date: NaiveDate,
    pub total_amount: Decimal,
}

#[derive(Serialize)]
pub struct InvoiceResponse {
    pub invoice: Invoice,
}

pub async fn handle_create_invoice(
    State(state): State<AppState>,
    Json(payload): Json<CreateInvoiceRequest>,
) -> Result<Json<InvoiceResponse>, AppError> {
    let invoice = create_invoice(
        &state.db,
        payload.business_id,
        payload.customer_id,
        &payload.number,
        payload.date,
        payload.due_date,
        payload.total_amount,
    ).await?;

    Ok(Json(InvoiceResponse { invoice }))
}
