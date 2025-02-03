use anyhow::Result;
use sqlx::PgPool;

use crate::model::{earning_history::EarningHistory, earning_history_pool::EarningHistoryPool};

pub struct EarningHistoryService {
    pool: PgPool,
}

impl EarningHistoryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_pools(
        &self,
        earning_history_pool: &[EarningHistoryPool],
        earning_history_id: i32,
    ) -> Result<usize> {
        let mut inserted: Vec<i32> = Vec::with_capacity(earning_history_pool.len());
        println!(
            "Inserted {} pools for earning_history_id:: {} ✅",
            earning_history_pool.len(),
            earning_history_id
        );
        for pool in earning_history_pool {
            let record = sqlx::query!(
                r#"
                    INSERT INTO pool_earnings(
                        earnings_history_id, pool, asset_liquidity_fees, 
                        rune_liquidity_fees, total_liquidity_fees_rune, 
                        saver_earning, rewards, earnings
                    )
                    VALUES($1, $2, $3, $4, $5, $6, $7, $8)
                    RETURNING id
                "#,
                earning_history_id,
                pool.pool,
                pool.asset_liquidity_fees,
                pool.rune_liquidity_fees,
                pool.total_liquidity_fees_rune,
                pool.saver_earning,
                pool.rewards,
                pool.earnings
            )
            .fetch_one(&self.pool)
            .await?;

            inserted.push(record.id);
        }
        Ok(inserted.len())
    }

    pub async fn save(&self, earning_history: &EarningHistory) -> Result<i32> {
        let result = sqlx::query!(
            r#"
            INSERT INTO earnings_history(
                start_time, end_time, liquidity_fees, block_rewards, 
                earnings, bonding_earnings, liquidity_earnings, 
                avg_node_count, rune_price_usd
            )
            VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id
            "#,
            earning_history.start_time,
            earning_history.end_time,
            earning_history.liquidity_fees,
            earning_history.block_rewards,
            earning_history.earnings,
            earning_history.bonding_earnings,
            earning_history.liquidity_earnings,
            earning_history.avg_node_count,
            earning_history.rune_price_usd
        )
        .fetch_one(&self.pool)
        .await?;

        let earning_history_pool: Vec<EarningHistoryPool> = earning_history
            .pools
            .iter()
            .cloned()
            .map(EarningHistoryPool::from)
            .collect();

        self.save_pools(earning_history_pool.as_slice(), result.id)
            .await?;

        Ok(result.id)
    }

    pub async fn save_batch(self, earning_histories: &[EarningHistory]) -> Result<Vec<i32>> {
        let mut ids = Vec::<i32>::with_capacity(earning_histories.len());

        for earning_history in earning_histories {
            // We have a list of pools (DTO), we'd need to convert it individually to a list of EarningHistoryPool model, then send that &[EarningHistoryPool] to the save method
            let id = self.save(earning_history).await?;
            ids.push(id);
        }

        Ok(ids)
    }
}
