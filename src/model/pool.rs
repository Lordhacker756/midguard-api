#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pool {
    pub pool: String,
    pub assetLiquidityFees: String,
    pub runeLiquidityFees: String,
    pub totalLiquidityFeesRune: String,
    pub saverEarning: String,
    pub rewards: String,
    pub earnings: String,
}
