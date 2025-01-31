CREATE TABLE IF NOT EXISTS runepool (
    id SERIAL PRIMARY KEY,                       
    start_time BIGINT,                           
    end_time BIGINT,                             
    units BIGINT,                                
    count BIGINT,                                
    UNIQUE (start_time, end_time)                
);