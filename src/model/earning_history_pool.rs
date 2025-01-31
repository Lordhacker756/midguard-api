#![allow(non_snake_case)]
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::conversions::deserialize_bigint;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EarningHistoryPool {
    pub pool: String,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub assetLiquidityFees: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub runeLiquidityFees: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub totalLiquidityFeesRune: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub saverEarning: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub rewards: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub earnings: BigInt,
}
