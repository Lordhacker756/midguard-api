use crate::{
    config::database::get_pool,
    model::rune_pool::{QueryParams, Runepool},
};
use anyhow::{Error, Result};
use axum::extract::Query;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};

pub struct RunePoolService<'a> {
    pool: &'a PgPool,
}

impl<'a> RunePoolService<'a> {
    pub fn new() -> Self {
        Self { pool: get_pool() }
    }

    pub async fn get_last_update_timestamp(&self) -> Result<i64, Error> {
        let record = sqlx::query!(
            "SELECT start_time FROM rune_pool_history ORDER BY start_time DESC LIMIT 1"
        )
        .fetch_one(self.pool)
        .await?;

        Ok(record.start_time.timestamp())
    }

    pub async fn get_all_runepools(
        &self,
        params: Query<QueryParams>,
    ) -> Result<Vec<Runepool>, Error> {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM rune_pool_history WHERE true");

        // Interval filter
        if let Some(interval) = &params.interval {
            let interval_trunc = match interval.as_str() {
                "5min" => "date_trunc('minute', start_time) + INTERVAL '5 minutes' * (EXTRACT(MINUTE FROM start_time)::int / 5)",
                "hour" => "date_trunc('hour', start_time)",
                "day" => "date_trunc('day', start_time)",
                "week" => "date_trunc('week', start_time)",
                "month" => "date_trunc('month', start_time)",
                "quarter" => "date_trunc('quarter', start_time)",
                "year" => "date_trunc('year', start_time)",
                _ => "date_trunc('hour', start_time)", // Default to hourly
            };

            qb.push(" AND start_time IN (");
            qb.push("SELECT DISTINCT ON (")
                .push(interval_trunc)
                .push(") start_time FROM rune_pool_history WHERE true");
            qb.push(")");
        }

        // Date range filter
        if let Some(date_range) = &params.date_range {
            let dates: Vec<&str> = date_range.split(',').collect();
            if dates.len() == 2 {
                qb.push(" AND start_time >= ")
                    .push("TO_TIMESTAMP(")
                    .push_bind(dates[0])
                    .push(", 'YYYY-MM-DD')")
                    .push(" AND end_time <= ")
                    .push("TO_TIMESTAMP(")
                    .push_bind(dates[1])
                    .push(", 'YYYY-MM-DD')");
            }
        }

        // Units filters
        if let Some(units_gt) = params.units_gt {
            qb.push(" AND units > ").push_bind(units_gt);
        }
        if let Some(units_lt) = params.units_lt {
            qb.push(" AND units < ").push_bind(units_lt);
        }
        if let Some(units_eq) = params.units_eq {
            qb.push(" AND units = ").push_bind(units_eq);
        }

        // Sorting
        if let Some(sort_by) = &params.sort_by {
            qb.push(" ORDER BY ").push(sort_by);

            if let Some(order) = &params.order {
                match order.to_lowercase().as_str() {
                    "asc" => qb.push(" ASC"),
                    "desc" => qb.push(" DESC"),
                    _ => qb.push(" ASC"), // default to ascending
                };
            }
        }

        // Pagination and limits
        if let Some(count) = params.count {
            qb.push(" LIMIT ").push_bind(count);
        } else if let Some(limit) = params.limit {
            qb.push(" LIMIT ").push_bind(limit);

            if let Some(page) = params.page {
                let offset = (page as i64).saturating_sub(1) * limit as i64;
                qb.push(" OFFSET ").push_bind(offset);
            }
        }

        let query = qb.build_query_as::<Runepool>();
        println!("SQL Query: {}", query.sql());
        let result = query.fetch_all(self.pool).await?;

        Ok(result)
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
        println!("📦 Batch saving {} rune pool records", rune_pools.len());

        let mut tx = self.pool.begin().await?;

        let copy = String::from(
            "COPY rune_pool_history (start_time, end_time, count, units) \
             FROM STDIN WITH (FORMAT text, DELIMITER '\t')",
        );

        let mut writer = tx.copy_in_raw(&copy).await?;

        // Process in chunks of 5000 records
        for (chunk_idx, chunk) in rune_pools.chunks(5000).enumerate() {
            println!("Processing chunk {}", chunk_idx + 1);
            let mut batch_data = String::with_capacity(chunk.len() * 256);

            for rune_pool in chunk {
                batch_data.push_str(&format!(
                    "{}\t{}\t{}\t{}\n",
                    rune_pool.start_time.format("%Y-%m-%d %H:%M:%S UTC"),
                    rune_pool.end_time.format("%Y-%m-%d %H:%M:%S UTC"),
                    rune_pool.count,
                    rune_pool.units
                ));
            }

            writer.send(batch_data.as_bytes()).await?;
        }

        writer.finish().await?;

        let ids = sqlx::query_as::<_, (i32,)>(
            "SELECT id FROM rune_pool_history ORDER BY id DESC LIMIT $1",
        )
        .bind(rune_pools.len() as i32)
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(|(id,)| id)
        .collect();

        tx.commit().await?;
        println!("✅ Successfully saved {} records", rune_pools.len());
        Ok(ids)
    }
}
