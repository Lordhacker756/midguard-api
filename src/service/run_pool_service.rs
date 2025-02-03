use crate::model::rune_pool::Runepool;
use anyhow::Result;
use sqlx::PgPool;

pub struct RunePoolService {
    pool: PgPool,
}

impl RunePoolService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
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
        .fetch_one(&self.pool)
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
