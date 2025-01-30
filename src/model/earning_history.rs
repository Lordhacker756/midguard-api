#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EarningInterval {
    pub startTime: String,
    pub endTime: String,
    pub liquidityFees: String,
    pub blockRewards: String,
    pub earnings: String,
    pub bondingEarnings: String,
    pub liquidityEarnings: String,
    pub avgNodeCount: String,
    pub runePriceUSD: String,
    pub pools: Vec<Pool>,
}
