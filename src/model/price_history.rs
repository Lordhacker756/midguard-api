#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
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
