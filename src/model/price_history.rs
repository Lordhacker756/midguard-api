#![allow(non_snake_case)]
use num_bigint::BigInt;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::conversions::{deserialize_bigint, deserialize_decimal};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PriceHistory {
    #[serde(deserialize_with = "deserialize_bigint")]
    pub assetDepth: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub assetPrice: Decimal,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub assetPriceUSD: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub endTime: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub liquidityUnits: BigInt,
    #[serde(deserialize_with = "deserialize_decimal")]
    pub luvi: Decimal,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub membersCount: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub runeDepth: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub startTime: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synthSupply: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub synthUnits: BigInt,
    #[serde(deserialize_with = "deserialize_bigint")]
    pub units: BigInt,
}
