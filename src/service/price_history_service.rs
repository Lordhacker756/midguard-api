use sqlx::PgPool;
use anyhow::Result;
use crate::model::price_history::PriceHistory;

pub struct PriceHistoryService {
    pool: PgPool,
}

impl PriceHistoryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save(&self, price_history: &PriceHistory) -> Result<i32> {
        let result = sqlx::query!(
            r#"
            INSERT INTO depth_price_history (
                start_time, end_time, asset_depth, rune_depth,
                asset_price, asset_price_usd, liquidity_units,
                members_count, synth_units, synth_supply,
                units, luvi
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id
            "#,
            price_history.start_time,
            price_history.end_time,
            price_history.asset_depth,
            price_history.rune_depth,
            price_history.asset_price,
            price_history.asset_price_usd,
            price_history.liquidity_units,
            price_history.members_count,
            price_history.synth_units,
            price_history.synth_supply,
            price_history.units,
            price_history.luvi
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.id)
    }

    pub async fn save_batch(&self, price_histories: &[PriceHistory]) -> Result<Vec<i32>> {
        let mut ids = Vec::with_capacity(price_histories.len());
        
        for price_history in price_histories {
            let id = self.save(price_history).await?;
            ids.push(id);
        }

        Ok(ids)
    }
}
