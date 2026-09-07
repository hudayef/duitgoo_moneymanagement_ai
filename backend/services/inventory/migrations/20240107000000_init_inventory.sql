CREATE TABLE products (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    sku VARCHAR(100) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DECIMAL(18, 2) NOT NULL DEFAULT 0.00,
    cost DECIMAL(18, 2) NOT NULL DEFAULT 0.00,
    track_inventory BOOLEAN NOT NULL DEFAULT true,
    current_stock DECIMAL(18, 4) NOT NULL DEFAULT 0.0000,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(business_id, sku)
);

CREATE TABLE stock_movements (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    product_id UUID NOT NULL REFERENCES products(id),
    movement_type VARCHAR(50) NOT NULL, -- IN, OUT, ADJUSTMENT
    quantity DECIMAL(18, 4) NOT NULL,
    reference_id UUID, -- E.g., Invoice ID or Bill ID
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_products_business_id ON products(business_id);
CREATE INDEX idx_stock_movements_business_id ON stock_movements(business_id);
CREATE INDEX idx_stock_movements_product_id ON stock_movements(product_id);
