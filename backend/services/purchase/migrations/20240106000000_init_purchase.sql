CREATE TABLE suppliers (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE bills (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    supplier_id UUID NOT NULL REFERENCES suppliers(id),
    number VARCHAR(100) NOT NULL,
    date DATE NOT NULL,
    due_date DATE NOT NULL,
    status VARCHAR(50) NOT NULL, -- DRAFT, RECEIVED, PARTIAL, PAID, OVERDUE, CANCELLED
    total_amount DECIMAL(18, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(business_id, supplier_id, number)
);

CREATE INDEX idx_suppliers_business_id ON suppliers(business_id);
CREATE INDEX idx_bills_business_id ON bills(business_id);
CREATE INDEX idx_bills_supplier_id ON bills(supplier_id);
