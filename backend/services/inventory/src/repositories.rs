use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{Product, StockMovement};
use shared::errors::AppError;

pub async fn create_product(
    pool: &PgPool,
    business_id: Uuid,
    sku: &str,
    name: &str,
    description: Option<String>,
    price: Decimal,
    cost: Decimal,
    track_inventory: bool,
) -> Result<Product, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO products (id, business_id, sku, name, description, price, cost, track_inventory)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *
    "#;

    let product = sqlx::query_as::<_, Product>(query)
        .bind(id)
        .bind(business_id)
        .bind(sku)
        .bind(name)
        .bind(description)
        .bind(price)
        .bind(cost)
        .bind(track_inventory)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create product: {}", e);
            AppError::InternalError("Could not create product".to_string())
        })?;

    Ok(product)
}

pub async fn adjust_stock(
    pool: &PgPool,
    business_id: Uuid,
    product_id: Uuid,
    movement_type: &str,
    quantity: Decimal,
    reference_id: Option<Uuid>,
    notes: Option<String>,
) -> Result<StockMovement, AppError> {
    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {}", e);
        AppError::InternalError("Database error".to_string())
    })?;

    // Validate product and get tracking status
    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = $1 AND business_id = $2 FOR UPDATE")
        .bind(product_id)
        .bind(business_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| AppError::NotFound)?;

    if !product.track_inventory {
        return Err(AppError::BadRequest("Product does not track inventory".to_string()));
    }

    let id = Uuid::new_v4();
    let query_mov = r#"
        INSERT INTO stock_movements (id, business_id, product_id, movement_type, quantity, reference_id, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
    "#;

    let movement = sqlx::query_as::<_, StockMovement>(query_mov)
        .bind(id)
        .bind(business_id)
        .bind(product_id)
        .bind(movement_type)
        .bind(quantity)
        .bind(reference_id)
        .bind(notes)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create stock movement: {}", e);
            AppError::InternalError("Could not log stock movement".to_string())
        })?;

    // Update product stock balance
    let qty_change = match movement_type {
        "IN" | "ADJUSTMENT" => quantity, // Positive adjustment means increase
        "OUT" => -quantity,
        _ => return Err(AppError::BadRequest("Invalid movement type".to_string())),
    };

    let new_stock = product.current_stock + qty_change;

    sqlx::query("UPDATE products SET current_stock = $1, updated_at = NOW() WHERE id = $2")
        .bind(new_stock)
        .bind(product_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update product stock: {}", e);
            AppError::InternalError("Could not update product stock".to_string())
        })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit stock transaction: {}", e);
        AppError::InternalError("Database transaction failed".to_string())
    })?;

    Ok(movement)
}
