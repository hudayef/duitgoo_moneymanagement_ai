use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use crate::{routes::AppState, repositories::*, models::*};
use shared::errors::AppError;

#[derive(Deserialize)]
pub struct CreateBankAccountRequest {
    pub business_id: Uuid,
    pub bank_name: String,
    pub account_number: String,
    pub account_name: String,
    pub currency: String,
}

#[derive(Serialize)]
pub struct BankAccountResponse {
    pub bank_account: BankAccount,
}

pub async fn handle_create_bank_account(
    State(state): State<AppState>,
    Json(payload): Json<CreateBankAccountRequest>,
) -> Result<Json<BankAccountResponse>, AppError> {
    let bank_account = create_bank_account(
        &state.db,
        payload.business_id,
        &payload.bank_name,
        &payload.account_number,
        &payload.account_name,
        &payload.currency
    ).await?;

    Ok(Json(BankAccountResponse { bank_account }))
}

#[derive(Deserialize)]
pub struct StartReconciliationRequest {
    pub business_id: Uuid,
    pub bank_account_id: Uuid,
    pub statement_date: NaiveDate,
    pub statement_balance: Decimal,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct ReconciliationResponse {
    pub reconciliation: Reconciliation,
}

pub async fn handle_start_reconciliation(
    State(state): State<AppState>,
    Json(payload): Json<StartReconciliationRequest>,
) -> Result<Json<ReconciliationResponse>, AppError> {
    let reconciliation = start_reconciliation(
        &state.db,
        payload.business_id,
        payload.bank_account_id,
        payload.statement_date,
        payload.statement_balance,
        payload.notes,
    ).await?;

    Ok(Json(ReconciliationResponse { reconciliation }))
}
