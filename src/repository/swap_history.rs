use crate::model::{responses::SwapInterval, swap_history::SwapHistory};
use num_bigint::BigInt;
use rust_decimal::Decimal;
use std::str::FromStr;

impl TryFrom<SwapInterval> for SwapHistory {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: SwapInterval) -> Result<Self, Self::Error> {
        Ok(SwapHistory {
            averageSlip: Decimal::from_str(&value.averageSlip)?,
            endTime: BigInt::from_str(&value.endTime)?,
            fromTradeAverageSlip: Decimal::from_str(&value.fromTradeAverageSlip)?,
            fromTradeCount: BigInt::from_str(&value.fromTradeCount)?,
            fromTradeFees: BigInt::from_str(&value.fromTradeFees)?,
            fromTradeVolume: BigInt::from_str(&value.fromTradeVolume)?,
            fromTradeVolumeUSD: Decimal::from_str(&value.fromTradeVolumeUSD)?,
            runePriceUSD: Decimal::from_str(&value.runePriceUSD)?,
            startTime: BigInt::from_str(&value.startTime)?,
            synthMintAverageSlip: Decimal::from_str(&value.synthMintAverageSlip)?,
            synthMintCount: BigInt::from_str(&value.synthMintCount)?,
            synthMintFees: BigInt::from_str(&value.synthMintFees)?,
            synthMintVolume: BigInt::from_str(&value.synthMintVolume)?,
            synthMintVolumeUSD: Decimal::from_str(&value.synthMintVolumeUSD)?,
            synthRedeemAverageSlip: Decimal::from_str(&value.synthRedeemAverageSlip)?,
            synthRedeemCount: BigInt::from_str(&value.synthRedeemCount)?,
            synthRedeemFees: BigInt::from_str(&value.synthRedeemFees)?,
            synthRedeemVolume: BigInt::from_str(&value.synthRedeemVolume)?,
            synthRedeemVolumeUSD: Decimal::from_str(&value.synthRedeemVolumeUSD)?,
            toAssetAverageSlip: Decimal::from_str(&value.toAssetAverageSlip)?,
            toAssetCount: BigInt::from_str(&value.toAssetCount)?,
            toAssetFees: BigInt::from_str(&value.toAssetFees)?,
            toAssetVolume: BigInt::from_str(&value.toAssetVolume)?,
            toAssetVolumeUSD: Decimal::from_str(&value.toAssetVolumeUSD)?,
            toRuneAverageSlip: Decimal::from_str(&value.toRuneAverageSlip)?,
            toRuneCount: BigInt::from_str(&value.toRuneCount)?,
            toRuneFees: BigInt::from_str(&value.toRuneFees)?,
            toRuneVolume: BigInt::from_str(&value.toRuneVolume)?,
            toRuneVolumeUSD: Decimal::from_str(&value.toRuneVolumeUSD)?,
            toTradeAverageSlip: Decimal::from_str(&value.toTradeAverageSlip)?,
            toTradeCount: BigInt::from_str(&value.toTradeCount)?,
            toTradeFees: BigInt::from_str(&value.toTradeFees)?,
            toTradeVolume: BigInt::from_str(&value.toTradeVolume)?,
            toTradeVolumeUSD: Decimal::from_str(&value.toTradeVolumeUSD)?,
            totalCount: BigInt::from_str(&value.totalCount)?,
            totalFees: BigInt::from_str(&value.totalFees)?,
            totalVolume: BigInt::from_str(&value.totalVolume)?,
            totalVolumeUSD: Decimal::from_str(&value.totalVolumeUSD)?,
        })
    }
}
