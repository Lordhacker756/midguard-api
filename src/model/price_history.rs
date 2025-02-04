use std::fmt;
use std::str::FromStr;

use crate::dtos::responses::PriceDepthInterval;
use chrono::{Date, DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Intervals {
    #[serde(rename = "5min")]
    FiveMin,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl FromStr for Intervals {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "5min" => Ok(Intervals::FiveMin),
            "hour" => Ok(Intervals::Hour),
            "day" => Ok(Intervals::Day),
            "week" => Ok(Intervals::Week),
            "month" => Ok(Intervals::Month),
            "quarter" => Ok(Intervals::Quarter),
            "year" => Ok(Intervals::Year),
            _ => Err(format!("Invalid interval: {}", s)),
        }
    }
}

impl fmt::Display for Intervals {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Intervals::FiveMin => write!(f, "5min"),
            Intervals::Hour => write!(f, "hour"),
            Intervals::Day => write!(f, "day"),
            Intervals::Week => write!(f, "week"),
            Intervals::Month => write!(f, "month"),
            Intervals::Quarter => write!(f, "quarter"),
            Intervals::Year => write!(f, "year"),
        }
    }
}

pub struct PriceHistoryParams {
    interval: Intervals,
    to: DateTime<Utc>,
    from: DateTime<Utc>,
    count: i8,
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
