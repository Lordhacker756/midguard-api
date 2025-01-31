use crate::model::{price_history::PriceHistory, responses::Interval};
use num_bigint::BigInt;
use rust_decimal::Decimal;
use std::str::FromStr;
    
impl TryFrom<Interval> for PriceHistory {
    type Error = Box<dyn std::error::Error>;

    fn try_from(interval: Interval) -> Result<Self, Self::Error> {
        Ok(PriceHistory {
            assetDepth: BigInt::from_str(&interval.assetDepth)?,
            assetPrice: Decimal::from_str(&interval.assetPrice)?,
            assetPriceUSD: Decimal::from_str(&interval.assetPriceUSD)?,
            endTime: BigInt::from_str(&interval.endTime)?,
            liquidityUnits: BigInt::from_str(&interval.liquidityUnits)?,
            luvi: Decimal::from_str(&interval.luvi)?,
            membersCount: BigInt::from_str(&interval.membersCount)?,
            runeDepth: BigInt::from_str(&interval.runeDepth)?,
            startTime: BigInt::from_str(&interval.startTime)?,
            synthSupply: BigInt::from_str(&interval.synthSupply)?,
            synthUnits: BigInt::from_str(&interval.synthUnits)?,
            units: BigInt::from_str(&interval.units)?,
        })
    }
}
