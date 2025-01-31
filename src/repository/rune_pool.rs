use num_bigint::BigInt;

use std::str::FromStr;

use crate::model::{responses::RunepoolInterval, rune_pool::Runepool};

impl TryFrom<RunepoolInterval> for Runepool {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: RunepoolInterval) -> Result<Self, Self::Error> {
        Ok(Runepool {
            count: BigInt::from_str(&value.count)?,
            startTime: BigInt::from_str(&value.startTime)?,
            endTime: BigInt::from_str(&value.endTime)?,
            units: BigInt::from_str(&value.units)?,
        })
    }
}
