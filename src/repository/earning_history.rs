use std::str::FromStr;

use num_bigint::BigInt;
use rust_decimal::Decimal;

use crate::model::{
    earning_history::EarningHistory, earning_history_pool::EarningHistoryPool,
    responses::EarningInterval,
};

impl TryFrom<EarningInterval> for EarningHistory {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: EarningInterval) -> Result<EarningHistory, Self::Error> {
        let pools = value
            .pools
            .into_iter()
            .map(EarningHistoryPool::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(EarningHistory {
            startTime: BigInt::from_str(&value.startTime)?,
            endTime: BigInt::from_str(&value.endTime)?,
            avgNodeCount: Decimal::from_str(&value.avgNodeCount)?,
            liquidityEarnings: BigInt::from_str(&value.liquidityEarnings)?,
            liquidityFees: BigInt::from_str(&value.liquidityFees)?,
            blockRewards: BigInt::from_str(&value.blockRewards)?,
            earnings: BigInt::from_str(&value.earnings)?,
            bondingEarnings: BigInt::from_str(&value.bondingEarnings)?,
            runePriceUSD: Decimal::from_str(&value.runePriceUSD)?,
            pools,
        })
    }
}
