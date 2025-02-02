use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthPriceHistoryResponse {
    pub intervals: Vec<PriceDepthInterval>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceDepthInterval {
    #[serde(rename = "assetDepth")]
    pub asset_depth: String,
    #[serde(rename = "assetPrice")]
    pub asset_price: String,
    #[serde(rename = "assetPriceUSD")]
    pub asset_price_usd: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "liquidityUnits")]
    pub liquidity_units: String,
    pub luvi: String,
    #[serde(rename = "membersCount")]
    pub members_count: String,
    #[serde(rename = "runeDepth")]
    pub rune_depth: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "synthSupply")]
    pub synth_supply: String,
    #[serde(rename = "synthUnits")]
    pub synth_units: String,
    pub units: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EarningHistoryResponse {
    pub intervals: Vec<EarningInterval>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pool {
    pub pool: String,
    #[serde(rename = "assetLiquidityFees")]
    pub asset_liquidity_fees: String,
    #[serde(rename = "runeLiquidityFees")]
    pub rune_liquidity_fees: String,
    #[serde(rename = "totalLiquidityFeesRune")]
    pub total_liquidity_fees_rune: String,
    #[serde(rename = "saverEarning")]
    pub saver_earning: String,
    pub rewards: String,
    pub earnings: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EarningInterval {
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "liquidityFees")]
    pub liquidity_fees: String,
    #[serde(rename = "blockRewards")]
    pub block_rewards: String,
    pub earnings: String,
    #[serde(rename = "bondingEarnings")]
    pub bonding_earnings: String,
    #[serde(rename = "liquidityEarnings")]
    pub liquidity_earnings: String,
    #[serde(rename = "avgNodeCount")]
    pub avg_node_count: String,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: String,
    pub pools: Vec<Pool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapHistoryResponse {
    pub intervals: Vec<SwapInterval>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapInterval {
    #[serde(rename = "averageSlip")]
    pub average_slip: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "fromSecuredAverageSlip")]
    pub from_secured_average_slip: String,
    #[serde(rename = "fromSecuredCount")]
    pub from_secured_count: String,
    #[serde(rename = "fromSecuredFees")]
    pub from_secured_fees: String,
    #[serde(rename = "fromSecuredVolume")]
    pub from_secured_volume: String,
    #[serde(rename = "fromSecuredVolumeUSD")]
    pub from_secured_volume_usd: String,
    #[serde(rename = "fromTradeAverageSlip")]
    pub from_trade_average_slip: String,
    #[serde(rename = "fromTradeCount")]
    pub from_trade_count: String,
    #[serde(rename = "fromTradeFees")]
    pub from_trade_fees: String,
    #[serde(rename = "fromTradeVolume")]
    pub from_trade_volume: String,
    #[serde(rename = "fromTradeVolumeUSD")]
    pub from_trade_volume_usd: String,
    #[serde(rename = "runePriceUSD")]
    pub rune_price_usd: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "synthMintAverageSlip")]
    pub synth_mint_average_slip: String,
    #[serde(rename = "synthMintCount")]
    pub synth_mint_count: String,
    #[serde(rename = "synthMintFees")]
    pub synth_mint_fees: String,
    #[serde(rename = "synthMintVolume")]
    pub synth_mint_volume: String,
    #[serde(rename = "synthMintVolumeUSD")]
    pub synth_mint_volume_usd: String,
    #[serde(rename = "synthRedeemAverageSlip")]
    pub synth_redeem_average_slip: String,
    #[serde(rename = "synthRedeemCount")]
    pub synth_redeem_count: String,
    #[serde(rename = "synthRedeemFees")]
    pub synth_redeem_fees: String,
    #[serde(rename = "synthRedeemVolume")]
    pub synth_redeem_volume: String,
    #[serde(rename = "synthRedeemVolumeUSD")]
    pub synth_redeem_volume_usd: String,
    #[serde(rename = "toAssetAverageSlip")]
    pub to_asset_average_slip: String,
    #[serde(rename = "toAssetCount")]
    pub to_asset_count: String,
    #[serde(rename = "toAssetFees")]
    pub to_asset_fees: String,
    #[serde(rename = "toAssetVolume")]
    pub to_asset_volume: String,
    #[serde(rename = "toAssetVolumeUSD")]
    pub to_asset_volume_usd: String,
    #[serde(rename = "toRuneAverageSlip")]
    pub to_rune_average_slip: String,
    #[serde(rename = "toRuneCount")]
    pub to_rune_count: String,
    #[serde(rename = "toRuneFees")]
    pub to_rune_fees: String,
    #[serde(rename = "toRuneVolume")]
    pub to_rune_volume: String,
    #[serde(rename = "toRuneVolumeUSD")]
    pub to_rune_volume_usd: String,
    #[serde(rename = "toSecuredAverageSlip")]
    pub to_secured_average_slip: String,
    #[serde(rename = "toSecuredCount")]
    pub to_secured_count: String,
    #[serde(rename = "toSecuredFees")]
    pub to_secured_fees: String,
    #[serde(rename = "toSecuredVolume")]
    pub to_secured_volume: String,
    #[serde(rename = "toSecuredVolumeUSD")]
    pub to_secured_volume_usd: String,
    #[serde(rename = "toTradeAverageSlip")]
    pub to_trade_average_slip: String,
    #[serde(rename = "toTradeCount")]
    pub to_trade_count: String,
    #[serde(rename = "toTradeFees")]
    pub to_trade_fees: String,
    #[serde(rename = "toTradeVolume")]
    pub to_trade_volume: String,
    #[serde(rename = "toTradeVolumeUSD")]
    pub to_trade_volume_usd: String,
    #[serde(rename = "totalCount")]
    pub total_count: String,
    #[serde(rename = "totalFees")]
    pub total_fees: String,
    #[serde(rename = "totalVolume")]
    pub total_volume: String,
    #[serde(rename = "totalVolumeUSD")]
    pub total_volume_usd: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunepoolHistoryResponse {
    pub intervals: Vec<RunepoolInterval>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunepoolInterval {
    pub count: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    pub units: String,
}
