use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::models::{Account, Journal};
use shared::errors::AppError;

pub async fn create_account(
    pool: &PgPool,
    business_id: Uuid,
    code: &str,
    name: &str,
    account_type: &str,
) -> Result<Account, AppError> {
    let id = Uuid::new_v4();
    let query = r#"
        INSERT INTO accounts (id, business_id, code, name, type)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
    "#;

    let account = sqlx::query_as::<_, Account>(query)
        .bind(id)
        .bind(business_id)
        .bind(code)
        .bind(name)
        .bind(account_type)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create account: {}", e);
            AppError::InternalError("Could not create account".to_string())
        })?;

    Ok(account)
}

pub struct NewJournalLine {
    pub account_id: Uuid,
    pub description: Option<String>,
    pub debit: Decimal,
    pub credit: Decimal,
}

pub async fn post_journal(
    pool: &PgPool,
    business_id: Uuid,
    date: chrono::NaiveDate,
    reference: Option<String>,
    description: Option<String>,
    idempotency_key: Option<String>,
    lines: Vec<NewJournalLine>,
) -> Result<Journal, AppError> {
    // 1. Enforce double-entry accounting rule (Debit == Credit)
    let total_debit: Decimal = lines.iter().map(|l| l.debit).sum();
    let total_credit: Decimal = lines.iter().map(|l| l.credit).sum();

    if total_debit != total_credit {
        return Err(AppError::BadRequest(format!(
            "Total debits ({}) must equal total credits ({})",
            total_debit, total_credit
        )));
    }

    if lines.is_empty() {
        return Err(AppError::BadRequest("Journal must have at least one line".to_string()));
    }

    // 2. Start atomic transaction
    let mut tx = pool.begin().await.map_err(|e| {
        tracing::error!("Failed to begin transaction: {}", e);
        AppError::InternalError("Database error".to_string())
    })?;

    // Idempotency check: if key exists, return error or existing journal (simplified to error here)
    if let Some(ref key) = idempotency_key {
        let existing = sqlx::query_as::<_, Journal>("SELECT * FROM journals WHERE idempotency_key = $1")
            .bind(key)
            .fetch_optional(&mut *tx)
            .await
            .unwrap_or(None);

        if existing.is_some() {
            // Rollback is automatic on drop
            return Err(AppError::BadRequest("Idempotency key already exists".to_string()));
        }
    }

    let journal_id = Uuid::new_v4();

    // 3. Insert Journal
    let query_journal = r#"
        INSERT INTO journals (id, business_id, date, reference, description, idempotency_key, status)
        VALUES ($1, $2, $3, $4, $5, $6, 'POSTED')
        RETURNING *
    "#;

    let journal = sqlx::query_as::<_, Journal>(query_journal)
        .bind(journal_id)
        .bind(business_id)
        .bind(date)
        .bind(reference)
        .bind(description)
        .bind(idempotency_key)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to insert journal: {}", e);
            AppError::InternalError("Could not post journal".to_string())
        })?;

    // 4. Insert Journal Lines & Update Account Balances
    for line in lines {
        let line_id = Uuid::new_v4();

        let query_line = r#"
            INSERT INTO journal_lines (id, journal_id, account_id, description, debit, credit)
            VALUES ($1, $2, $3, $4, $5, $6)
        "#;

        sqlx::query(query_line)
            .bind(line_id)
            .bind(journal_id)
            .bind(line.account_id)
            .bind(line.description)
            .bind(line.debit)
            .bind(line.credit)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to insert journal line: {}", e);
                AppError::InternalError("Could not insert journal line".to_string())
            })?;

        // Calculate balance impact. Simplification: Assets/Expenses increase with Debit, Liabilities/Equity/Revenue increase with Credit.
        // We will just do a standard arithmetic for now or update a net_balance field.
        // For standard double entry reporting, we update the balance:
        // Asset/Expense: balance = balance + debit - credit
        // Liability/Equity/Revenue: balance = balance - debit + credit

        // Let's get the account type first
        let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE id = $1 AND business_id = $2")
            .bind(line.account_id)
            .bind(business_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|_| AppError::BadRequest("Invalid account ID or business ID".to_string()))?;

        let new_balance = match account.r#type.as_str() {
            "ASSET" | "EXPENSE" => account.balance + line.debit - line.credit,
            "LIABILITY" | "EQUITY" | "REVENUE" => account.balance - line.debit + line.credit,
            _ => account.balance,
        };

        sqlx::query("UPDATE accounts SET balance = $1, updated_at = NOW() WHERE id = $2")
            .bind(new_balance)
            .bind(account.id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to update account balance: {}", e);
                AppError::InternalError("Could not update balance".to_string())
            })?;
    }

    // 5. Commit transaction
    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit journal transaction: {}", e);
        AppError::InternalError("Database transaction failed".to_string())
    })?;

    Ok(journal)
}
