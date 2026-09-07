use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use crate::{routes::AppState, repositories::*, models::*};
use shared::errors::AppError;

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    pub business_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
}

#[derive(Serialize)]
pub struct AccountResponse {
    pub account: Account,
}

pub async fn create_acc(
    State(state): State<AppState>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, AppError> {
    let account = create_account(&state.db, payload.business_id, &payload.code, &payload.name, &payload.account_type).await?;
    Ok(Json(AccountResponse { account }))
}

#[derive(Deserialize)]
pub struct LineItemInput {
    pub account_id: Uuid,
    pub description: Option<String>,
    pub debit: Decimal,
    pub credit: Decimal,
}

#[derive(Deserialize)]
pub struct PostJournalRequest {
    pub business_id: Uuid,
    pub date: NaiveDate,
    pub reference: Option<String>,
    pub description: Option<String>,
    pub idempotency_key: Option<String>,
    pub lines: Vec<LineItemInput>,
}

#[derive(Serialize)]
pub struct JournalResponse {
    pub journal: Journal,
}

pub async fn post_txn(
    State(state): State<AppState>,
    Json(payload): Json<PostJournalRequest>,
) -> Result<Json<JournalResponse>, AppError> {
    let mut new_lines = Vec::new();
    for l in payload.lines {
        new_lines.push(NewJournalLine {
            account_id: l.account_id,
            description: l.description,
            debit: l.debit,
            credit: l.credit,
        });
    }

    let journal = post_journal(
        &state.db,
        payload.business_id,
        payload.date,
        payload.reference,
        payload.description,
        payload.idempotency_key,
        new_lines,
    ).await?;

    Ok(Json(JournalResponse { journal }))
}
