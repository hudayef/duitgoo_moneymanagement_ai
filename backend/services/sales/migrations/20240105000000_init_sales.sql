CREATE TABLE customers (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE invoices (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    customer_id UUID NOT NULL REFERENCES customers(id),
    number VARCHAR(100) NOT NULL,
    date DATE NOT NULL,
    due_date DATE NOT NULL,
    status VARCHAR(50) NOT NULL, -- DRAFT, SENT, PARTIAL, PAID, OVERDUE, CANCELLED
    total_amount DECIMAL(18, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(business_id, number)
);

CREATE INDEX idx_customers_business_id ON customers(business_id);
CREATE INDEX idx_invoices_business_id ON invoices(business_id);
CREATE INDEX idx_invoices_customer_id ON invoices(customer_id);
