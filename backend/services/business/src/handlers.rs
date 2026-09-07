use axum::{Json, extract::{State, Path}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{routes::AppState, repositories::{create_business, get_user_businesses}, models::Business};
use shared::errors::AppError;

#[derive(Deserialize)]
pub struct CreateBusinessRequest {
    pub owner_id: Uuid, // In reality, extract from JWT
    pub name: String,
}

#[derive(Serialize)]
pub struct BusinessResponse {
    pub business: Business,
}

#[derive(Serialize)]
pub struct BusinessListResponse {
    pub businesses: Vec<Business>,
}

pub async fn create_biz(
    State(state): State<AppState>,
    Json(payload): Json<CreateBusinessRequest>,
) -> Result<Json<BusinessResponse>, AppError> {
    if payload.name.is_empty() {
         return Err(AppError::BadRequest("Business name is required".to_string()));
    }

    let business = create_business(&state.db, payload.owner_id, &payload.name).await?;

    Ok(Json(BusinessResponse { business }))
}

pub async fn list_biz(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<BusinessListResponse>, AppError> {
    let businesses = get_user_businesses(&state.db, user_id).await?;

    Ok(Json(BusinessListResponse { businesses }))
}
