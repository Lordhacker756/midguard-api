use num_bigint::BigInt;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer};

/// Custom deserializer for BigInt
pub fn deserialize_bigint<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<BigInt>().map_err(serde::de::Error::custom)
}

/// Custom deserializer for Decimal
pub fn deserialize_decimal<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<Decimal>().map_err(serde::de::Error::custom)
}
