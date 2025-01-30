#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthPriceHistoryResponse {
    pub intervals: Vec<Interval>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interval {
    pub assetDepth: String,
    pub assetPrice: String,
    pub assetPriceUSD: String,
    pub endTime: String,
    pub liquidityUnits: String,
    pub luvi: String,
    pub membersCount: String,
    pub runeDepth: String,
    pub startTime: String,
    pub synthSupply: String,
    pub synthUnits: String,
    pub units: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningHistoryResponse {
    pub intervals: Vec<EarningInterval>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pool {
    pub pool: String,
    pub assetLiquidityFees: String,
    pub runeLiquidityFees: String,
    pub totalLiquidityFeesRune: String,
    pub saverEarning: String,
    pub rewards: String,
    pub earnings: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningInterval {
    pub startTime: String,
    pub endTime: String,
    pub liquidityFees: String,
    pub blockRewards: String,
    pub earnings: String,
    pub bondingEarnings: String,
    pub liquidityEarnings: String,
    pub avgNodeCount: String,
    pub runePriceUSD: String,
    pub pools: Vec<Pool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapHistoryResponse {
    pub intervals: Vec<SwapInterval>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapInterval {
    pub averageSlip: String,
    pub endTime: String,
    pub fromSecuredAverageSlip: String,
    pub fromSecuredCount: String,
    pub fromSecuredFees: String,
    pub fromSecuredVolume: String,
    pub fromSecuredVolumeUSD: String,
    pub fromTradeAverageSlip: String,
    pub fromTradeCount: String,
    pub fromTradeFees: String,
    pub fromTradeVolume: String,
    pub fromTradeVolumeUSD: String,
    pub runePriceUSD: String,
    pub startTime: String,
    pub synthMintAverageSlip: String,
    pub synthMintCount: String,
    pub synthMintFees: String,
    pub synthMintVolume: String,
    pub synthMintVolumeUSD: String,
    pub synthRedeemAverageSlip: String,
    pub synthRedeemCount: String,
    pub synthRedeemFees: String,
    pub synthRedeemVolume: String,
    pub synthRedeemVolumeUSD: String,
    pub toAssetAverageSlip: String,
    pub toAssetCount: String,
    pub toAssetFees: String,
    pub toAssetVolume: String,
    pub toAssetVolumeUSD: String,
    pub toRuneAverageSlip: String,
    pub toRuneCount: String,
    pub toRuneFees: String,
    pub toRuneVolume: String,
    pub toRuneVolumeUSD: String,
    pub toSecuredAverageSlip: String,
    pub toSecuredCount: String,
    pub toSecuredFees: String,
    pub toSecuredVolume: String,
    pub toSecuredVolumeUSD: String,
    pub toTradeAverageSlip: String,
    pub toTradeCount: String,
    pub toTradeFees: String,
    pub toTradeVolume: String,
    pub toTradeVolumeUSD: String,
    pub totalCount: String,
    pub totalFees: String,
    pub totalVolume: String,
    pub totalVolumeUSD: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunepoolHistoryResponse {
    pub intervals: Vec<RunepoolInterval>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunepoolInterval {
    pub count: String,
    pub startTime: String,
    pub endTime: String,
    pub units: String,
}
