use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use crate::{routes::AppState, repositories::*, models::*};
use shared::errors::AppError;

#[derive(Deserialize)]
pub struct CreateBudgetRequest {
    pub business_id: Uuid,
    pub name: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_budget: Decimal,
}

#[derive(Serialize)]
pub struct BudgetResponse {
    pub budget: Budget,
}

pub async fn handle_create_budget(
    State(state): State<AppState>,
    Json(payload): Json<CreateBudgetRequest>,
) -> Result<Json<BudgetResponse>, AppError> {
    let budget = create_budget(
        &state.db,
        payload.business_id,
        &payload.name,
        payload.period_start,
        payload.period_end,
        payload.total_budget
    ).await?;

    Ok(Json(BudgetResponse { budget }))
}

#[derive(Deserialize)]
pub struct AddBudgetLineRequest {
    pub budget_id: Uuid,
    pub account_id: Uuid,
    pub allocated_amount: Decimal,
}

#[derive(Serialize)]
pub struct BudgetLineResponse {
    pub budget_line: BudgetLine,
}

pub async fn handle_add_budget_line(
    State(state): State<AppState>,
    Json(payload): Json<AddBudgetLineRequest>,
) -> Result<Json<BudgetLineResponse>, AppError> {
    let budget_line = add_budget_line(
        &state.db,
        payload.budget_id,
        payload.account_id,
        payload.allocated_amount,
    ).await?;

    Ok(Json(BudgetLineResponse { budget_line }))
}
