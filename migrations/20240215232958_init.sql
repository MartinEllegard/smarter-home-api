-- Add migration script here
CREATE TABLE IF NOT EXISTS 'live_consumptions' (
  home_id UUID NOT NULL,
  timestamp TIMESTAMPTZ NOT NULL,
  power DOUBLE PRECISION NOT NULL,
  min_power DOUBLE PRECISION NOT NULL,
  max_power DOUBLE PRECISION NOT NULL,
  average_power DOUBLE PRECISION NOT NULL,
  last_meter_consumption DOUBLE PRECISION NOT NULL,
  last_meter_production DOUBLE PRECISION NOT NULL,
  accumulated_consumption_today DOUBLE PRECISION NOT NULL,
  accumulated_production_today DOUBLE PRECISION NOT NULL,
  accumulated_consumption_hour DOUBLE PRECISION NOT NULL,
  accumulated_production_hour DOUBLE PRECISION NOT NULL,
  current_price DOUBLE PRECISION NOT NULL,
  accumulated_cost_today DOUBLE PRECISION NOT NULL
);

CREATE TABLE 'electricity_prices' (
  home_id UUID NOT NULL,
  currency TEXT NOT NULL,
  timestamp TIMESTAMPTZ NOT NULL,
  total_by_provider DOUBLE PRECISION NOT NULL,
  spot DOUBLE PRECISION NOT NULL,
  tax DOUBLE PRECISION NOT NULL,
  calculated DOUBLE PRECISION NOT NULL,
  grid DOUBLE PRECISION NOT NULL
);
