CREATE TABLE IF NOT EXISTS earnings_history (
  id SERIAL PRIMARY KEY,
  start_time TIMESTAMPTZ NOT NULL,
  end_time TIMESTAMPTZ NOT NULL,
  liquidity_fees BIGINT NOT NULL,
  block_rewards BIGINT NOT NULL,
  earnings BIGINT NOT NULL,
  bonding_earnings BIGINT NOT NULL,
  liquidity_earnings BIGINT NOT NULL,
  avg_node_count NUMERIC NOT NULL,
  rune_price_usd NUMERIC NOT NULL
);
CREATE TABLE IF NOT EXISTS pool_earnings (
  id SERIAL PRIMARY KEY,
  earnings_history_id SERIAL NOT NULL REFERENCES earnings_history(id) ON DELETE CASCADE,
  pool TEXT NOT NULL,
  asset_liquidity_fees BIGINT NOT NULL,
  rune_liquidity_fees BIGINT NOT NULL,
  total_liquidity_fees_rune BIGINT NOT NULL,
  saver_earning BIGINT NOT NULL,
  rewards BIGINT NOT NULL,
  earnings BIGINT NOT NULL
);