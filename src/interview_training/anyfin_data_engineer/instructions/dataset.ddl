CREATE TABLE dataset (
	application_id TEXT,
    created_at TIMESTAMP,
    customer_id INTEGER ,
    applications_before INTEGER,
    loans_before INTEGER,
    paid_cycles_before INTEGER,
    avg_dpd_30_days_before FLOAT,
    max_dpd_30_days_before FLOAT,
    avg_dpd_60_days_before FLOAT,
    max_dpd_60_days_before FLOAT
);
