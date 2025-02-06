use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::dtos::responses::EarningInterval;

use super::earning_history_pool::EarningHistoryPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    pub interval: Option<String>,
    pub limit: Option<i16>,
    pub page: Option<i16>,
    pub order: Option<String>,
    pub sort_by: Option<String>,
    pub date_range: Option<String>,
    pub count: Option<i8>,

    // liquidity fees
    pub liquidity_fees_lt: Option<i64>,
    pub liquidity_fees_eq: Option<i64>,
    pub liquidity_fees_gt: Option<i64>,

    // block rewards
    pub block_rewards_lt: Option<i64>,
    pub block_rewards_eq: Option<i64>,
    pub block_rewards_gt: Option<i64>,

    // earnings
    pub earnings_lt: Option<i64>,
    pub earnings_eq: Option<i64>,
    pub earnings_gt: Option<i64>,

    // bonding earnings
    pub bonding_earnings_lt: Option<i64>,
    pub bonding_earnings_eq: Option<i64>,
    pub bonding_earnings_gt: Option<i64>,

    // liquidity earnings
    pub liquidity_earnings_lt: Option<i64>,
    pub liquidity_earnings_eq: Option<i64>,
    pub liquidity_earnings_gt: Option<i64>,

    // avg node count
    pub avg_node_count_lt: Option<Decimal>,
    pub avg_node_count_eq: Option<Decimal>,
    pub avg_node_count_gt: Option<Decimal>,

    // rune price usd
    pub rune_price_usd_lt: Option<Decimal>,
    pub rune_price_usd_eq: Option<Decimal>,
    pub rune_price_usd_gt: Option<Decimal>,

    // Pools Filters
    pub pool_eq: Option<String>,
    pub pool_asset_liquidity_fees_lt: Option<i32>,
    pub pool_asset_liquidity_fees_eq: Option<i32>,
    pub pool_asset_liquidity_fees_gt: Option<i32>,
    pub pool_rune_liquidity_fees_lt: Option<i32>,
    pub pool_rune_liquidity_fees_eq: Option<i32>,
    pub pool_rune_liquidity_fees_gt: Option<i32>,
    pub pool_total_liquidity_fees_lt: Option<i32>,
    pub pool_total_liquidity_fees_eq: Option<i32>,
    pub pool_total_liquidity_fees_gt: Option<i32>,
    pub pool_saver_earning_lt: Option<i32>,
    pub pool_saver_earning_eq: Option<i32>,
    pub pool_saver_earning_gt: Option<i32>,
    pub pool_rewards_lt: Option<i32>,
    pub pool_rewards_eq: Option<i32>,
    pub pool_rewards_gt: Option<i32>,
    pub pool_earnings_lt: Option<i32>,
    pub pool_earnings_eq: Option<i32>,
    pub pool_earnings_gt: Option<i32>,
}

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
    pub pools: Option<Vec<EarningHistoryPool>>,
}

impl From<EarningInterval> for EarningHistory {
    fn from(value: EarningInterval) -> Self {
        let earnign_pools = value
            .pools
            .iter()
            .map(|pool| EarningHistoryPool {
                id: None,
                earnings_history_id: None,
                pool: pool.pool.clone(),
                asset_liquidity_fees: pool.asset_liquidity_fees.parse().unwrap_or_default(),
                rune_liquidity_fees: pool.rune_liquidity_fees.parse().unwrap_or_default(),
                total_liquidity_fees_rune: pool
                    .total_liquidity_fees_rune
                    .parse()
                    .unwrap_or_default(),
                saver_earning: pool.saver_earning.parse().unwrap_or_default(),
                rewards: pool.rewards.parse().unwrap_or_default(),
                earnings: pool.earnings.parse().unwrap_or_default(),
            })
            .collect();
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
            pools: Some(earnign_pools),
        }
    }
}
