use serde::{Deserialize, Serialize};

use crate::dtos::responses::Pool;

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningHistoryPool {
    pub id: Option<i32>,
    pub earnings_history_id: Option<i32>,
    pub pool: String,
    pub asset_liquidity_fees: i64,
    pub rune_liquidity_fees: i64,
    pub total_liquidity_fees_rune: i64,
    pub saver_earning: i64,
    pub rewards: i64,
    pub earnings: i64,
}

impl From<Pool> for EarningHistoryPool {
    fn from(value: Pool) -> Self {
        Self {
            id: None,
            asset_liquidity_fees: value.asset_liquidity_fees.parse().unwrap_or(0),
            earnings_history_id: None,
            pool: value.pool,
            rune_liquidity_fees: value.rune_liquidity_fees.parse().unwrap_or(0),
            total_liquidity_fees_rune: value.total_liquidity_fees_rune.parse().unwrap_or(0),
            saver_earning: value.saver_earning.parse().unwrap_or(0),
            rewards: value.rewards.parse().unwrap_or(0),
            earnings: value.earnings.parse().unwrap_or(0),
        }
    }
}
