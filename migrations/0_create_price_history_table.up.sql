CREATE TABLE IF NOT EXISTS price_history (
    id SERIAL PRIMARY KEY,                 
    asset_depth BIGINT,                     
    asset_price DECIMAL(18, 8),             
    asset_price_usd DECIMAL(18, 8),         
    liquidity_units BIGINT,                 
    luvi DECIMAL(18, 8),                    
    members_count BIGINT,                   
    rune_depth BIGINT,                      
    synth_supply BIGINT,                    
    synth_units BIGINT,                     
    units BIGINT,                           
    start_time BIGINT,                      
    end_time BIGINT,                        
    UNIQUE (start_time, end_time)           
);