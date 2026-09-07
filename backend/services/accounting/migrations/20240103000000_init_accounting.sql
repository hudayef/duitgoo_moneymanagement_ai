CREATE TABLE accounts (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL, -- ASSET, LIABILITY, EQUITY, REVENUE, EXPENSE
    balance DECIMAL(18, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(business_id, code)
);

CREATE TABLE journals (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    date DATE NOT NULL,
    reference VARCHAR(255),
    description TEXT,
    idempotency_key VARCHAR(255) UNIQUE,
    status VARCHAR(50) NOT NULL, -- DRAFT, POSTED, VOIDED
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE journal_lines (
    id UUID PRIMARY KEY,
    journal_id UUID NOT NULL REFERENCES journals(id) ON DELETE CASCADE,
    account_id UUID NOT NULL REFERENCES accounts(id),
    description TEXT,
    debit DECIMAL(18, 2) NOT NULL DEFAULT 0.00,
    credit DECIMAL(18, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_accounts_business_id ON accounts(business_id);
CREATE INDEX idx_journals_business_id ON journals(business_id);
CREATE INDEX idx_journal_lines_journal_id ON journal_lines(journal_id);
