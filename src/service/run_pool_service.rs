use crate::{
    config::database::get_pool,
    model::rune_pool::{QueryParams, Runepool},
};
use anyhow::{Error, Result};
use axum::extract::Query;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder, Row};

pub struct RunePoolService<'a> {
    pool: &'a PgPool,
}

impl<'a> RunePoolService<'a> {
    pub fn new() -> Self {
        Self { pool: get_pool() }
    }

    pub async fn get_all_runepools(
        &self,
        params: Query<QueryParams>,
    ) -> Result<Vec<Runepool>, Error> {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM rune_pool_history WHERE true");

        // TODO
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
                .push(") start_time FROM rune_pool_history");
            qb.push(" WHERE true"); // Subquery filtering
            qb.push(")");
        }

        if let Some(date_range) = &params.date_range {
            let dates: Vec<&str> = date_range.split(',').collect();
            qb.push(" AND start_time >= ")
                .push("TO_TIMESTAMP(")
                .push_bind(dates[0])
                .push(", 'YYYY-MM-DD')")
                .push(" AND end_time <= ")
                .push("TO_TIMESTAMP(")
                .push_bind(dates[1])
                .push(", 'YYYY-MM-DD')");
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
                qb.push(" ").push(order.to_lowercase());
            }
        }

        // Count limit
        if let Some(count) = params.count {
            qb.push(" LIMIT ").push_bind(count);
        }

        // Pagination
        if let Some(limit) = params.limit {
            qb.push(" LIMIT ").push_bind(limit);
        }
        if let Some(page) = params.page {
            let offset = page as i64 * params.limit.unwrap_or(10) as i64;
            qb.push(" OFFSET ").push_bind(offset);
        }

        // Execute the query
        let query = qb.build();
        println!("SQL Query: {}", query.sql());
        let result = query.fetch_all(self.pool).await?;

        Ok(result
            .into_iter()
            .map(|record| Runepool {
                id: record.get("id"),
                start_time: record.get("start_time"),
                end_time: record.get("end_time"),
                count: record.get("count"),
                units: record.get("units"),
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
