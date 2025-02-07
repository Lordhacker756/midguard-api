use crate::{
    config::database::get_pool,
    error::AppError,
    model::rune_pool::{QueryParams, Runepool},
};
use axum::extract::Query;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder, Row};

pub struct RunePoolService<'a> {
    pool: &'a PgPool,
}

impl<'a> RunePoolService<'a> {
    pub fn new() -> Result<Self, AppError> {
        println!("📊 Initializing RunePoolService");
        Ok(Self {
            pool: get_pool().map_err(|e| {
                AppError::new(format!("Failed to initialize database pool: {:#?}", e))
            })?,
        })
    }

    pub async fn get_last_update_timestamp(&self) -> Result<i64, AppError> {
        let record = sqlx::query(
            "SELECT start_time FROM rune_pool_history ORDER BY start_time DESC LIMIT 1",
        )
        .fetch_one(self.pool)
        .await
        .map_err(|e| AppError::new(format!("Failed to get last timestamp: {}", e)))?;

        Ok(record
            .get::<chrono::DateTime<chrono::Utc>, _>("start_time")
            .timestamp())
    }

    pub async fn get_all_runepools(
        &self,
        params: Query<QueryParams>,
    ) -> Result<Vec<Runepool>, AppError> {
        println!("🔍 Fetching rune pools with params: {:?}", params);
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
        println!("🔎 Executing SQL Query: {}", query.sql());
        let result = query
            .fetch_all(self.pool)
            .await
            .map_err(|e| AppError::new(format!("Failed to fetch rune pools: {}", e)))?;

        println!("✅ Found {} records", result.len());
        Ok(result)
    }

    pub async fn save(&self, rune_pool: &Runepool) -> Result<i32, AppError> {
        println!("💾 Saving single rune pool record");
        let result = sqlx::query(
            r#"
                INSERT INTO rune_pool_history (
                    start_time, end_time, count, units
                )
                VALUES ($1, $2, $3, $4)
                RETURNING id
            "#,
        )
        .bind(&rune_pool.start_time)
        .bind(&rune_pool.end_time)
        .bind(&rune_pool.count)
        .bind(&rune_pool.units)
        .fetch_one(self.pool)
        .await
        .map_err(|e| AppError::new(format!("Failed to save rune pool: {}", e)))?;

        let id = result.get::<i32, _>("id");
        println!("✅ Saved record with ID: {}", id);
        Ok(id)
    }

    pub async fn save_batch(&self, rune_pools: &[Runepool]) -> Result<Vec<i32>, AppError> {
        if rune_pools.is_empty() {
            return Err(AppError::new("No rune pools provided for batch save"));
        }

        println!("📦 Batch saving {} rune pool records", rune_pools.len());

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| AppError::new(format!("Failed to start transaction: {}", e)))?;

        let copy = String::from(
            "COPY rune_pool_history (start_time, end_time, count, units) \
             FROM STDIN WITH (FORMAT text, DELIMITER '\t')",
        );

        let mut writer = tx
            .copy_in_raw(&copy)
            .await
            .map_err(|e| AppError::new(format!("Failed to initialize batch write: {}", e)))?;

        // Process in chunks of 5000 records
        for (chunk_idx, chunk) in rune_pools.chunks(5000).enumerate() {
            println!(
                "Processing chunk {} of {}",
                chunk_idx + 1,
                (rune_pools.len() + 4999) / 5000
            );
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

            writer
                .send(batch_data.as_bytes())
                .await
                .map_err(|e| AppError::new(format!("Failed to write batch data: {}", e)))?;
        }

        writer
            .finish()
            .await
            .map_err(|e| AppError::new(format!("Failed to finish batch write: {}", e)))?;

        let ids = sqlx::query_as::<_, (i32,)>(
            "SELECT id FROM rune_pool_history ORDER BY id DESC LIMIT $1",
        )
        .bind(rune_pools.len() as i32)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| AppError::new(format!("Failed to retrieve inserted IDs: {}", e)))?;

        tx.commit()
            .await
            .map_err(|e| AppError::new(format!("Failed to commit transaction: {}", e)))?;

        let ids = ids.into_iter().map(|(id,)| id).collect();
        println!("🎉 Successfully saved {} records", rune_pools.len());
        Ok(ids)
    }
}
