use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::dtos::responses::SwapInterval;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SwapHistory {
    pub id: Option<i32>,
    pub average_slip: Decimal,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub from_trade_average_slip: Decimal,
    pub from_trade_count: i64,
    pub from_trade_fees: i64,
    pub from_trade_volume: i64,
    pub from_trade_volume_usd: Decimal,
    pub rune_price_usd: Decimal,
    pub synth_mint_average_slip: Decimal,
    pub synth_mint_count: i64,
    pub synth_mint_fees: i64,
    pub synth_mint_volume: i64,
    pub synth_mint_volume_usd: Decimal,
    pub synth_redeem_average_slip: Decimal,
    pub synth_redeem_count: i64,
    pub synth_redeem_fees: i64,
    pub synth_redeem_volume: i64,
    pub synth_redeem_volume_usd: Decimal,
    pub to_asset_average_slip: Decimal,
    pub to_asset_count: i64,
    pub to_asset_fees: i64,
    pub to_asset_volume: i64,
    pub to_asset_volume_usd: Decimal,
    pub to_rune_average_slip: Decimal,
    pub to_rune_count: i64,
    pub to_rune_fees: i64,
    pub to_rune_volume: i64,
    pub to_rune_volume_usd: Decimal,
    pub to_trade_average_slip: Decimal,
    pub to_trade_count: i64,
    pub to_trade_fees: i64,
    pub to_trade_volume: i64,
    pub to_trade_volume_usd: Decimal,
    pub total_count: i64,
    pub total_fees: i64,
    pub total_volume: i64,
    pub total_volume_usd: Decimal,
}

impl From<SwapInterval> for SwapHistory {
    fn from(value: SwapInterval) -> Self {
        Self {
            id: None,
            average_slip: value.average_slip.parse::<Decimal>().unwrap_or_default(),
            start_time: DateTime::parse_from_rfc3339(&value.start_time)
                .unwrap_or_default()
                .with_timezone(&Utc),
            end_time: DateTime::parse_from_rfc3339(&value.end_time)
                .unwrap_or_default()
                .with_timezone(&Utc),
            from_trade_average_slip: value.from_trade_average_slip.parse().unwrap_or_default(),
            from_trade_count: value.from_trade_count.parse().unwrap_or(0),
            from_trade_fees: value.from_trade_fees.parse().unwrap_or(0),
            from_trade_volume: value.from_trade_volume.parse().unwrap_or(0),
            from_trade_volume_usd: value.from_trade_volume_usd.parse().unwrap_or_default(),
            rune_price_usd: value.rune_price_usd.parse().unwrap_or_default(),
            synth_mint_average_slip: value.synth_mint_average_slip.parse().unwrap_or_default(),
            synth_mint_count: value.synth_mint_count.parse().unwrap_or(0),
            synth_mint_fees: value.synth_mint_fees.parse().unwrap_or(0),
            synth_mint_volume: value.synth_mint_volume.parse().unwrap_or(0),
            synth_mint_volume_usd: value.synth_mint_volume_usd.parse().unwrap_or_default(),
            synth_redeem_average_slip: value.synth_redeem_average_slip.parse().unwrap_or_default(),
            synth_redeem_count: value.synth_redeem_count.parse().unwrap_or(0),
            synth_redeem_fees: value.synth_redeem_fees.parse().unwrap_or(0),
            synth_redeem_volume: value.synth_redeem_volume.parse().unwrap_or(0),
            synth_redeem_volume_usd: value.synth_redeem_volume_usd.parse().unwrap_or_default(),
            to_asset_average_slip: value.to_asset_average_slip.parse().unwrap_or_default(),
            to_asset_count: value.to_asset_count.parse().unwrap_or(0),
            to_asset_fees: value.to_asset_fees.parse().unwrap_or(0),
            to_asset_volume: value.to_asset_volume.parse().unwrap_or(0),
            to_asset_volume_usd: value.to_asset_volume_usd.parse().unwrap_or_default(),
            to_rune_average_slip: value.to_rune_average_slip.parse().unwrap_or_default(),
            to_rune_count: value.to_rune_count.parse().unwrap_or(0),
            to_rune_fees: value.to_rune_fees.parse().unwrap_or(0),
            to_rune_volume: value.to_rune_volume.parse().unwrap_or(0),
            to_rune_volume_usd: value.to_rune_volume_usd.parse().unwrap_or_default(),
            to_trade_average_slip: value.to_trade_average_slip.parse().unwrap_or_default(),
            to_trade_count: value.to_trade_count.parse().unwrap_or(0),
            to_trade_fees: value.to_trade_fees.parse().unwrap_or(0),
            to_trade_volume: value.to_trade_volume.parse().unwrap_or(0),
            to_trade_volume_usd: value.to_trade_volume_usd.parse().unwrap_or_default(),
            total_count: value.total_count.parse().unwrap_or(0),
            total_fees: value.total_fees.parse().unwrap_or(0),
            total_volume: value.total_volume.parse().unwrap_or(0),
            total_volume_usd: value.total_volume_usd.parse().unwrap_or_default(),
        }
    }
}
