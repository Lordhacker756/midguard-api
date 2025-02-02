#![allow(non_snake_case)]
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::utils::conversions::deserialize_bigint;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Runepool {
    #[serde(deserialize_with = "deserialize_bigint")]
    pub count: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub start_time: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub end_time: BigInt,

    #[serde(deserialize_with = "deserialize_bigint")]
    pub units: BigInt,
}
