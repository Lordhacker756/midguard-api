#![allow(non_snake_case)]
use num_bigint::BigInt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::conversions::{deserialize_bigint, deserialize_decimal};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SwapHistory {
    #[serde(deserialize_with = "deserialize_decimal")]
    pub average_slip: Decimal,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub end_time: BigInt,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub from_trade_average_slip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub from_trade_count: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub from_trade_fees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub from_trade_volume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub from_trade_volume_usd: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub rune_price_usd: Decimal,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub start_time: BigInt,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub synth_mint_average_slip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synth_mint_count: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synth_mint_fees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synth_mint_volume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub synth_mint_volume_usd: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub synth_redeem_average_slip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synth_redeem_count: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synth_redeem_fees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synth_redeem_volume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub synth_redeem_volume_usd: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub to_asset_average_slip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub to_asset_count: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub to_asset_fees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub to_asset_volume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub to_asset_volume_usd: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub to_rune_average_slip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub to_rune_count: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub to_rune_fees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub to_rune_volume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub to_rune_volume_usd: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub to_trade_average_slip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub to_trade_count: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub to_trade_fees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub to_trade_volume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub to_trade_volume_usd: Decimal,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub total_count: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub total_fees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub total_volume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub total_volume_usd: Decimal,
}
