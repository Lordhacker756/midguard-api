#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthPriceHistoryResponse {
    pub intervals: Vec<Interval>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interval {
    pub assetDepth: String,
    pub assetPrice: String,
    pub assetPriceUSD: String,
    pub endTime: String,
    pub liquidityUnits: String,
    pub luvi: String,
    pub membersCount: String,
    pub runeDepth: String,
    pub startTime: String,
    pub synthSupply: String,
    pub synthUnits: String,
    pub units: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningHistoryResponse {
    pub intervals: Vec<EarningInterval>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pool {
    pub pool: String,
    pub assetLiquidityFees: String,
    pub runeLiquidityFees: String,
    pub totalLiquidityFeesRune: String,
    pub saverEarning: String,
    pub rewards: String,
    pub earnings: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
