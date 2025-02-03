use crate::dtos::responses::PriceDepthInterval;
use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct PriceHistory {
    pub id: Option<i32>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub asset_depth: i64,
    pub rune_depth: i64,
    pub asset_price: Decimal,
    pub asset_price_usd: Decimal,
    pub liquidity_units: i64,
    pub members_count: i64,
    pub synth_units: i64,
    pub synth_supply: i64,
    pub units: i64,
    pub luvi: Decimal,
}

impl From<PriceDepthInterval> for PriceHistory {
    fn from(interval: PriceDepthInterval) -> Self {
        Self {
            id: None,
            start_time: Utc
                .timestamp_opt(interval.start_time.parse().unwrap_or(0), 0)
                .unwrap(),
            end_time: Utc
                .timestamp_opt(interval.end_time.parse().unwrap_or(0), 0)
                .unwrap(),
            asset_depth: interval.asset_depth.parse().unwrap_or(0),
            rune_depth: interval.rune_depth.parse().unwrap_or(0),
            asset_price: interval.asset_price.parse().unwrap_or(Decimal::ZERO),
            asset_price_usd: interval.asset_price_usd.parse().unwrap_or(Decimal::ZERO),
            liquidity_units: interval.liquidity_units.parse().unwrap_or(0),
            members_count: interval.members_count.parse().unwrap_or(0),
            synth_units: interval.synth_units.parse().unwrap_or(0),
            synth_supply: interval.synth_supply.parse().unwrap_or(0),
            units: interval.units.parse().unwrap_or(0),
            luvi: interval.luvi.parse().unwrap_or(Decimal::ZERO),
        }
    }
}
