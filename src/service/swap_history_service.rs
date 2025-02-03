use crate::model::swap_history::SwapHistory;
use anyhow::Result;
use sqlx::PgPool;

pub struct SwapHistoryService {
    pool: PgPool,
}

impl SwapHistoryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, swap_history: &SwapHistory) -> Result<i32> {
        let result = sqlx::query!(r#"
                INSERT INTO swap_history (
                            average_slip, end_time, from_trade_average_slip, from_trade_count, from_trade_fees, from_trade_volume, from_trade_volume_usd,
                            rune_price_usd, start_time, synth_mint_average_slip, synth_mint_count, synth_mint_fees, synth_mint_volume, synth_mint_volume_usd,
                            synth_redeem_average_slip, synth_redeem_count, synth_redeem_fees, synth_redeem_volume, synth_redeem_volume_usd,
                            to_asset_average_slip, to_asset_count, to_asset_fees, to_asset_volume, to_asset_volume_usd,
                            to_rune_average_slip, to_rune_count, to_rune_fees, to_rune_volume, to_rune_volume_usd,
                            to_trade_average_slip, to_trade_count, to_trade_fees, to_trade_volume, to_trade_volume_usd,
                            total_count, total_fees, total_volume, total_volume_usd
                            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, 
                            $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38
                )
                RETURNING id
            "#,
            swap_history.average_slip,
            swap_history.end_time,
            swap_history.from_trade_average_slip,
            swap_history.from_trade_count,
            swap_history.from_trade_fees,
            swap_history.from_trade_volume,
            swap_history.from_trade_volume_usd,
            swap_history.rune_price_usd,
            swap_history.start_time,
            swap_history.synth_mint_average_slip,
            swap_history.synth_mint_count,
            swap_history.synth_mint_fees,
            swap_history.synth_mint_volume,
            swap_history.synth_mint_volume_usd,
            swap_history.synth_redeem_average_slip,
            swap_history.synth_redeem_count,
            swap_history.synth_redeem_fees,
            swap_history.synth_redeem_volume,
            swap_history.synth_redeem_volume_usd,
            swap_history.to_asset_average_slip,
            swap_history.to_asset_count,
            swap_history.to_asset_fees,
            swap_history.to_asset_volume,
            swap_history.to_asset_volume_usd,
            swap_history.to_rune_average_slip,
            swap_history.to_rune_count,
            swap_history.to_rune_fees,
            swap_history.to_rune_volume,
            swap_history.to_rune_volume_usd,
            swap_history.to_trade_average_slip,
            swap_history.to_trade_count,
            swap_history.to_trade_fees,
            swap_history.to_trade_volume,
            swap_history.to_trade_volume_usd,
            swap_history.total_count,
            swap_history.total_fees,
            swap_history.total_volume,
            swap_history.total_volume_usd)
            .fetch_one(&self.pool)
            .await?;

        Ok(result.id)
    }

    pub async fn save_batch(&self, swap_historys: &[SwapHistory]) -> Result<Vec<i32>> {
        let mut ids = Vec::with_capacity(swap_historys.len());

        for swap_history in swap_historys {
            let id = self.save(swap_history).await?;
            ids.push(id);
        }

        Ok(ids)
    }
}
