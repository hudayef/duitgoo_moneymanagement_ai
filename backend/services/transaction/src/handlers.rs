use axum::{Json, extract::{State, Path}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{routes::AppState, repositories::*, models::*};
use shared::errors::AppError;

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub business_id: Uuid,
    pub transaction_type: String,
    pub idempotency_key: Option<String>,
    pub reference: Option<String>,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub transaction: Transaction,
}

pub async fn create_txn(
    State(state): State<AppState>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, AppError> {
    let transaction = create_transaction(
        &state.db,
        payload.business_id,
        &payload.transaction_type,
        payload.idempotency_key,
        payload.reference,
    ).await?;

    // Example: Publish event to NATS
    // let event = BaseEvent::new("transaction.created", transaction.business_id, &transaction, None);
    // state.publisher.publish("transactions.events", &event).await?;

    Ok(Json(TransactionResponse { transaction }))
}

#[derive(Deserialize)]
pub struct UpdateTransactionRequest {
    pub status: String,
}

pub async fn update_txn(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTransactionRequest>,
) -> Result<Json<TransactionResponse>, AppError> {
    let transaction = update_transaction_status(&state.db, id, &payload.status).await?;
    Ok(Json(TransactionResponse { transaction }))
}
