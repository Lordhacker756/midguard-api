use crate::model::{earning_history_pool::EarningHistoryPool, responses::Pool};
use num_bigint::BigInt;
use std::str::FromStr;

impl TryFrom<Pool> for EarningHistoryPool {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Pool) -> Result<Self, Self::Error> {
        Ok(EarningHistoryPool {
            pool: value.pool,
            assetLiquidityFees: BigInt::from_str(&value.assetLiquidityFees)?,
            runeLiquidityFees: BigInt::from_str(&value.runeLiquidityFees)?,
            totalLiquidityFeesRune: BigInt::from_str(&value.totalLiquidityFeesRune)?,
            saverEarning: BigInt::from_str(&value.saverEarning)?,
            rewards: BigInt::from_str(&value.rewards)?,
            earnings: BigInt::from_str(&value.earnings)?,
        })
    }
}
