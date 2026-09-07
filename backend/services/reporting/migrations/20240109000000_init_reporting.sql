CREATE TABLE financial_reports (
    id UUID PRIMARY KEY,
    business_id UUID NOT NULL,
    report_type VARCHAR(50) NOT NULL, -- TRIAL_BALANCE, PROFIT_LOSS, BALANCE_SHEET, CASH_FLOW
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    report_data JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_financial_reports_business_id ON financial_reports(business_id);
CREATE INDEX idx_financial_reports_type_period ON financial_reports(business_id, report_type, period_start, period_end);
