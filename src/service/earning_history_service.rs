use sqlx::{Pool, Postgres, Result};

use crate::model::earning_history::EarningHistory;

pub struct EarningHistoryService {
    pool: Pool<Postgres>,
}

impl EarningHistoryService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    //* Create
    // pub async fn create(&self, history: &EarningHistory) -> Result<EarningHistory> {
    //     let record = sqlx::query_as!(
    //         EarningHistory,
    //         r#"
    //     INSERT INTO earning_history (
    //         "startTime", "endTime", "liquidityFees",
    //         "blockRewards", "earnings", "bondingEarnings",
    //         "liquidityEarnings", "avgNodeCount", "runePriceUSD"
    //     )
    //     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    //     RETURNING *
    //     "#,
    //         history.startTime,
    //         history.endTime,
    //         history.liquidityFees,
    //         history.blockRewards,
    //         history.earnings,
    //         history.bondingEarnings,
    //         history.liquidityEarnings,
    //         history.avgNodeCount,
    //         history.runePriceUSD
    //     )
    //     .fetch_one(&self.pool)
    //     .await?;

    //     Ok(record)
    // }
    //TODO: Update

    //? Read

    //Delete
}
