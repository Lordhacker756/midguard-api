#![allow(non_snake_case)]
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::conversions::deserialize_bigint;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct EarningHistoryPool {
    pub pool: String,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub asset_liquidity_fees: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub rune_liquidity_fees: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub total_liquidity_fees_rune: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub saver_earning: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub rewards: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub earnings: BigInt,
}
