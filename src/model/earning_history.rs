use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::dtos::responses::{EarningInterval, Pool};

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningHistory {
    pub id: Option<i32>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub liquidity_fees: i64,
    pub block_rewards: i64,
    pub earnings: i64,
    pub bonding_earnings: i64,
    pub liquidity_earnings: i64,
    pub avg_node_count: Decimal,
    pub rune_price_usd: Decimal,
    pub pools: Vec<Pool>,
}

impl From<EarningInterval> for EarningHistory {
    fn from(value: EarningInterval) -> Self {
        Self {
            id: None,
            start_time: Utc
                .timestamp_opt(value.start_time.parse().unwrap_or(0), 0)
                .unwrap(),
            end_time: Utc
                .timestamp_opt(value.end_time.parse().unwrap_or(0), 0)
                .unwrap(),
            avg_node_count: value.avg_node_count.parse().unwrap_or(Decimal::ZERO),
            block_rewards: value.block_rewards.parse().unwrap_or(0),
            bonding_earnings: value.bonding_earnings.parse().unwrap_or(0),
            earnings: value.earnings.parse().unwrap_or(0),
            liquidity_earnings: value.liquidity_earnings.parse().unwrap_or(0),
            liquidity_fees: value.liquidity_fees.parse().unwrap_or(0),
            rune_price_usd: value.rune_price_usd.parse().unwrap_or(Decimal::ZERO),
            pools: value.pools,
        }
    }
}
