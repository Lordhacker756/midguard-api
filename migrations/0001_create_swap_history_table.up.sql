CREATE TABLE IF NOT EXISTS swap_history (
    id SERIAL PRIMARY KEY,
    average_slip NUMERIC NOT NULL,
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    
    from_trade_average_slip NUMERIC NOT NULL,
    from_trade_count BIGINT NOT NULL,
    from_trade_fees BIGINT NOT NULL,
    from_trade_volume BIGINT NOT NULL,
    from_trade_volume_usd NUMERIC NOT NULL,
    
    rune_price_usd NUMERIC NOT NULL,
    
    synth_mint_average_slip NUMERIC NOT NULL,
    synth_mint_count BIGINT NOT NULL,
    synth_mint_fees BIGINT NOT NULL,
    synth_mint_volume BIGINT NOT NULL,
    synth_mint_volume_usd NUMERIC NOT NULL,
    
    synth_redeem_average_slip NUMERIC NOT NULL,
    synth_redeem_count BIGINT NOT NULL,
    synth_redeem_fees BIGINT NOT NULL,
    synth_redeem_volume BIGINT NOT NULL,
    synth_redeem_volume_usd NUMERIC NOT NULL,
    
    to_asset_average_slip NUMERIC NOT NULL,
    to_asset_count BIGINT NOT NULL,
    to_asset_fees BIGINT NOT NULL,
    to_asset_volume BIGINT NOT NULL,
    to_asset_volume_usd NUMERIC NOT NULL,
    
    to_rune_average_slip NUMERIC NOT NULL,
    to_rune_count BIGINT NOT NULL,
    to_rune_fees BIGINT NOT NULL,
    to_rune_volume BIGINT NOT NULL,
    to_rune_volume_usd NUMERIC NOT NULL,
    
    to_trade_average_slip NUMERIC NOT NULL,
    to_trade_count BIGINT NOT NULL,
    to_trade_fees BIGINT NOT NULL,
    to_trade_volume BIGINT NOT NULL,
    to_trade_volume_usd NUMERIC NOT NULL,
    
    total_count BIGINT NOT NULL,
    total_fees BIGINT NOT NULL,
    total_volume BIGINT NOT NULL,
    total_volume_usd NUMERIC NOT NULL
);