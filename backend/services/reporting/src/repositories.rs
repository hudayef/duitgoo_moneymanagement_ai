use sqlx::PgPool;
use uuid::Uuid;
use crate::models::FinancialReport;
use shared::errors::AppError;

pub async fn save_report(
    pool: &PgPool,
    business_id: Uuid,
    report_type: &str,
    period_start: chrono::NaiveDate,
    period_end: chrono::NaiveDate,
    report_data: serde_json::Value,
) -> Result<FinancialReport, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO financial_reports (id, business_id, report_type, period_start, period_end, report_data)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
    "#;

    let report = sqlx::query_as::<_, FinancialReport>(query)
        .bind(id)
        .bind(business_id)
        .bind(report_type)
        .bind(period_start)
        .bind(period_end)
        .bind(sqlx::types::Json(report_data))
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to save report: {}", e);
            AppError::InternalError("Could not save financial report".to_string())
        })?;

    Ok(report)
}

pub async fn get_report(
    pool: &PgPool,
    business_id: Uuid,
    report_type: &str,
    period_start: chrono::NaiveDate,
    period_end: chrono::NaiveDate,
) -> Result<Option<FinancialReport>, AppError> {
    let query = r#"
        SELECT * FROM financial_reports
        WHERE business_id = $1 AND report_type = $2 AND period_start = $3 AND period_end = $4
        ORDER BY created_at DESC LIMIT 1
    "#;

    let report = sqlx::query_as::<_, FinancialReport>(query)
        .bind(business_id)
        .bind(report_type)
        .bind(period_start)
        .bind(period_end)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch report: {}", e);
            AppError::InternalError("Could not fetch financial report".to_string())
        })?;

    Ok(report)
}
