use crate::{config::database::get_pool, model::rune_pool::Runepool};
use anyhow::{Error, Result};
use sqlx::PgPool;

pub struct RunePoolService<'a> {
    pool: &'a PgPool,
}

impl<'a> RunePoolService<'a> {
    pub fn new() -> Self {
        Self { pool: get_pool() }
    }

    pub async fn get_all_runepools(&self) -> Result<Vec<Runepool>, Error> {
        let result = sqlx::query!(
            r#"
                SELECT * FROM rune_pool_history
            "#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(result
            .into_iter()
            .map(|result| Runepool {
                id: Some(result.id),
                count: result.count,
                start_time: result.start_time,
                end_time: result.end_time,
                units: result.units,
            })
            .collect())
    }

    pub async fn save(&self, rune_pool: &Runepool) -> Result<i32> {
        let result = sqlx::query!(
            r#"
                INSERT INTO rune_pool_history (
                    start_time, end_time, count, units
                )
                VALUES ($1, $2, $3, $4)
                RETURNING id
            "#,
            rune_pool.start_time,
            rune_pool.end_time,
            rune_pool.count,
            rune_pool.units
        )
        .fetch_one(self.pool)
        .await?;

        Ok(result.id)
    }

    pub async fn save_batch(&self, rune_pools: &[Runepool]) -> Result<Vec<i32>> {
        let mut ids = Vec::with_capacity(rune_pools.len());

        for rune_pool in rune_pools {
            let id = self.save(rune_pool).await?;
            ids.push(id);
        }

        Ok(ids)
    }
}
