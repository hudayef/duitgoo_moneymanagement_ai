CREATE TABLE bank_accounts (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    bank_name VARCHAR(255) NOT NULL,
    account_number VARCHAR(100) NOT NULL,
    account_name VARCHAR(255) NOT NULL,
    currency VARCHAR(10) NOT NULL DEFAULT 'IDR',
    balance DECIMAL(18, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(business_id, bank_name, account_number)
);

CREATE TABLE reconciliations (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    bank_account_id UUID NOT NULL REFERENCES bank_accounts(id),
    statement_date DATE NOT NULL,
    statement_balance DECIMAL(18, 2) NOT NULL,
    status VARCHAR(50) NOT NULL, -- DRAFT, IN_PROGRESS, RECONCILED
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bank_accounts_business_id ON bank_accounts(business_id);
CREATE INDEX idx_reconciliations_business_id ON reconciliations(business_id);
CREATE INDEX idx_reconciliations_bank_account_id ON reconciliations(bank_account_id);
