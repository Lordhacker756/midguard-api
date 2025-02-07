CREATE TABLE IF NOT EXISTS depth_price_history (
  id SERIAL PRIMARY KEY,
  start_time TIMESTAMPTZ NOT NULL,
  end_time TIMESTAMPTZ NOT NULL,
  asset_depth BIGINT NOT NULL,
  rune_depth BIGINT NOT NULL,
  asset_price NUMERIC NOT NULL,
  asset_price_usd NUMERIC NOT NULL,
  liquidity_units BIGINT NOT NULL,
  members_count BIGINT NOT NULL,
  synth_units BIGINT NOT NULL,
  synth_supply BIGINT NOT NULL,
  units BIGINT NOT NULL,
  luvi NUMERIC NOT NULL
);