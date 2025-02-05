use anyhow::{Error, Result};
use sqlx::PgPool;

use crate::{
    config::database::get_pool,
    model::{earning_history::EarningHistory, earning_history_pool::EarningHistoryPool},
};

pub struct EarningHistoryService<'a> {
    pool: &'a PgPool,
}

impl<'a> EarningHistoryService<'a> {
    pub fn new() -> Self {
        Self { pool: get_pool() }
    }

    pub async fn get_all_pools(&self, earning_history_id: i32) -> Result<Vec<EarningHistoryPool>> {
        let pools = sqlx::query!(
            r#"
            SELECT * FROM pool_earnings WHERE earnings_history_id = $1
        "#,
            earning_history_id
        )
        .fetch_all(self.pool)
        .await?;

        Ok(pools
            .into_iter()
            .map(|record| EarningHistoryPool {
                id: Some(record.id),
                earnings_history_id: Some(record.earnings_history_id),
                pool: record.pool,
                asset_liquidity_fees: record.asset_liquidity_fees,
                rune_liquidity_fees: record.rune_liquidity_fees,
                total_liquidity_fees_rune: record.total_liquidity_fees_rune,
                saver_earning: record.saver_earning,
                rewards: record.rewards,
                earnings: record.earnings,
            })
            .collect())
    }

    pub async fn get_all_earnings_history(&self) -> Result<Vec<EarningHistory>, Error> {
        let earning_histories = sqlx::query!(
            r#"
            SELECT * FROM earnings_history
            "#
        )
        .fetch_all(self.pool)
        .await?;

        let mut earnings = earning_histories
            .into_iter()
            .map(|record| EarningHistory {
                id: Some(record.id),
                start_time: record.start_time,
                end_time: record.end_time,
                liquidity_fees: record.liquidity_fees,
                block_rewards: record.block_rewards,
                earnings: record.earnings,
                bonding_earnings: record.bonding_earnings,
                liquidity_earnings: record.liquidity_earnings,
                avg_node_count: record.avg_node_count,
                rune_price_usd: record.rune_price_usd,
                pools: None,
            })
            .collect::<Vec<EarningHistory>>();

        for earning in &mut earnings {
            let pools = self.get_all_pools(earning.id.unwrap()).await?;
            earning.pools = Some(pools);
        }

        Ok(earnings)
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
            .fetch_one(self.pool)
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
        .fetch_one(self.pool)
        .await?;

        if let Some(pools) = &earning_history.pools {
            self.save_pools(pools.as_slice(), result.id).await?;
        }

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
