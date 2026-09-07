use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;
use crate::{routes::AppState, repositories::*, models::*};
use shared::errors::AppError;

#[derive(Deserialize)]
pub struct GenerateReportRequest {
    pub business_id: Uuid,
    pub report_type: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

#[derive(Serialize)]
pub struct ReportResponse {
    pub report: FinancialReport,
}

pub async fn handle_generate_report(
    State(state): State<AppState>,
    Json(payload): Json<GenerateReportRequest>,
) -> Result<Json<ReportResponse>, AppError> {
    // 1. Check if cached report exists
    if let Some(cached) = get_report(&state.db, payload.business_id, &payload.report_type, payload.period_start, payload.period_end).await? {
        return Ok(Json(ReportResponse { report: cached }));
    }

    // 2. Generate new report data (Placeholder logic - Phase 10)
    // Real implementation would calculate aggregates based on NATS events or read-model projections.
    let generated_data = serde_json::json!({
        "status": "draft",
        "aggregates": {
            "total_assets": 0.00,
            "total_liabilities": 0.00
        }
    });

    // 3. Save generated report
    let report = save_report(
        &state.db,
        payload.business_id,
        &payload.report_type,
        payload.period_start,
        payload.period_end,
        generated_data,
    ).await?;

    Ok(Json(ReportResponse { report }))
}
