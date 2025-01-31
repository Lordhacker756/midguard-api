#![allow(non_snake_case)]
use num_bigint::BigInt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::model::earning_history_pool::EarningHistoryPool;
use crate::utils::conversions::{deserialize_bigint, deserialize_decimal};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EarningHistory {
    #[serde(deserialize_with = "deserialize_bigint")]
    pub startTime: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub endTime: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub liquidityFees: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub blockRewards: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub earnings: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub bondingEarnings: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub liquidityEarnings: BigInt,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub avgNodeCount: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub runePriceUSD: Decimal,

    pub pools: Vec<EarningHistoryPool>,
}
