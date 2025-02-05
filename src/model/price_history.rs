use std::fmt;
use std::str::FromStr;

use crate::dtos::responses::PriceDepthInterval;
use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceHistoryParams {
    pub interval: Option<String>,
    pub limit: Option<i16>,
    pub page: Option<i16>,
    pub order: Option<String>,
    pub sort_by: Option<String>,
    pub date_range: Option<String>,
    pub count: Option<i8>,

    // Asset depth filters
    pub asset_depth_gt: Option<i64>,
    pub asset_depth_lt: Option<i64>,
    pub asset_depth_eq: Option<i64>,

    // Rune depth filters
    pub rune_depth_gt: Option<i64>,
    pub rune_depth_lt: Option<i64>,
    pub rune_depth_eq: Option<i64>,

    // Price filters
    pub asset_price_gt: Option<Decimal>,
    pub asset_price_lt: Option<Decimal>,
    pub asset_price_eq: Option<Decimal>,

    // USD price filters
    pub asset_price_usd_gt: Option<Decimal>,
    pub asset_price_usd_lt: Option<Decimal>,
    pub asset_price_usd_eq: Option<Decimal>,

    // Liquidity units filters
    pub liquidity_units_gt: Option<i64>,
    pub liquidity_units_lt: Option<i64>,
    pub liquidity_units_eq: Option<i64>,

    // Members count filters
    pub members_count_gt: Option<i64>,
    pub members_count_lt: Option<i64>,
    pub members_count_eq: Option<i64>,

    // Synth related filters
    pub synth_units_gt: Option<i64>,
    pub synth_units_lt: Option<i64>,
    pub synth_units_eq: Option<i64>,

    pub synth_supply_gt: Option<i64>,
    pub synth_supply_lt: Option<i64>,
    pub synth_supply_eq: Option<i64>,

    // Units filters
    pub units_gt: Option<i64>,
    pub units_lt: Option<i64>,
    pub units_eq: Option<i64>,

    // LUVI filters
    pub luvi_gt: Option<Decimal>,
    pub luvi_lt: Option<Decimal>,
    pub luvi_eq: Option<Decimal>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
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
