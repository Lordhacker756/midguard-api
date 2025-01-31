#![allow(non_snake_case)]
use num_bigint::BigInt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::conversions::{deserialize_bigint, deserialize_decimal};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SwapHistory {
    #[serde(deserialize_with = "deserialize_decimal")]
    pub averageSlip: Decimal,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub endTime: BigInt,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub fromTradeAverageSlip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub fromTradeCount: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub fromTradeFees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub fromTradeVolume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub fromTradeVolumeUSD: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub runePriceUSD: Decimal,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub startTime: BigInt,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub synthMintAverageSlip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synthMintCount: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synthMintFees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synthMintVolume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub synthMintVolumeUSD: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub synthRedeemAverageSlip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synthRedeemCount: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synthRedeemFees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synthRedeemVolume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub synthRedeemVolumeUSD: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub toAssetAverageSlip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub toAssetCount: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub toAssetFees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub toAssetVolume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub toAssetVolumeUSD: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub toRuneAverageSlip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub toRuneCount: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub toRuneFees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub toRuneVolume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub toRuneVolumeUSD: Decimal,

    #[serde(deserialize_with = "deserialize_decimal")]
    pub toTradeAverageSlip: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub toTradeCount: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub toTradeFees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub toTradeVolume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub toTradeVolumeUSD: Decimal,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub totalCount: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub totalFees: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub totalVolume: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub totalVolumeUSD: Decimal,
}
