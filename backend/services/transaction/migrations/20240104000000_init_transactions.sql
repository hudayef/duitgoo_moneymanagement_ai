CREATE TABLE transactions (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    transaction_type VARCHAR(50) NOT NULL, -- SALE, PURCHASE, TRANSFER
    status VARCHAR(50) NOT NULL, -- PENDING, COMPLETED, FAILED, VOIDED
    idempotency_key VARCHAR(255) UNIQUE,
    reference VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_transactions_business_id ON transactions(business_id);
