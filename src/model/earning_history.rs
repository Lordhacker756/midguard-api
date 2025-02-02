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
    pub start_time: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub end_time: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub liquidity_fees: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub block_rewards: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub earnings: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub bonding_earnings: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub liquidity_earnings: BigInt,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub avg_node_count: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub rune_price_usd: Decimal,

    pub pools: Vec<EarningHistoryPool>,
}
