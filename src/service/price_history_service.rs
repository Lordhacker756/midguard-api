use crate::{
    config::database::get_pool,
    model::price_history::{PriceHistory, PriceHistoryParams},
};
use anyhow::{Error, Result};
use axum::extract::Query;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder, Row};

pub struct PriceHistoryService<'a> {
    pool: &'a PgPool,
}

impl<'a> PriceHistoryService<'a> {
    pub fn new() -> Self {
        Self { pool: get_pool() }
    }

    pub async fn get_all_price_history(
        &self,
        params: Query<PriceHistoryParams>,
    ) -> Result<Vec<PriceHistory>, Error> {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM depth_price_history WHERE true");
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
                .push(") start_time FROM depth_price_history");
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

        // Asset Depth filters
        if let Some(asset_depth_gt) = params.asset_depth_gt {
            qb.push(" AND asset_depth > ").push_bind(asset_depth_gt);
        }
        if let Some(asset_depth_lt) = params.asset_depth_lt {
            qb.push(" AND asset_depth < ").push_bind(asset_depth_lt);
        }
        if let Some(asset_depth_eq) = params.asset_depth_eq {
            qb.push(" AND asset_depth = ").push_bind(asset_depth_eq);
        }

        // Rune Depth filters
        if let Some(rune_depth_gt) = params.rune_depth_gt {
            qb.push(" AND rune_depth > ").push_bind(rune_depth_gt);
        }
        if let Some(rune_depth_lt) = params.rune_depth_lt {
            qb.push(" AND rune_depth < ").push_bind(rune_depth_lt);
        }
        if let Some(rune_depth_eq) = params.rune_depth_eq {
            qb.push(" AND rune_depth = ").push_bind(rune_depth_eq);
        }

        // Asset Price filters
        if let Some(asset_price_gt) = params.asset_price_gt {
            qb.push(" AND asset_price > ").push_bind(asset_price_gt);
        }
        if let Some(asset_price_lt) = params.asset_price_lt {
            qb.push(" AND asset_price < ").push_bind(asset_price_lt);
        }
        if let Some(asset_price_eq) = params.asset_price_eq {
            qb.push(" AND asset_price = ").push_bind(asset_price_eq);
        }

        // USD Price filters
        if let Some(asset_price_usd_gt) = params.asset_price_usd_gt {
            qb.push(" AND asset_price_usd > ")
                .push_bind(asset_price_usd_gt);
        }
        if let Some(asset_price_usd_lt) = params.asset_price_usd_lt {
            qb.push(" AND asset_price_usd < ")
                .push_bind(asset_price_usd_lt);
        }
        if let Some(asset_price_usd_eq) = params.asset_price_usd_eq {
            qb.push(" AND asset_price_usd = ")
                .push_bind(asset_price_usd_eq);
        }

        // Liquidity Units filters
        if let Some(liquidity_units_gt) = params.liquidity_units_gt {
            qb.push(" AND liquidity_units > ")
                .push_bind(liquidity_units_gt);
        }
        if let Some(liquidity_units_lt) = params.liquidity_units_lt {
            qb.push(" AND liquidity_units < ")
                .push_bind(liquidity_units_lt);
        }
        if let Some(liquidity_units_eq) = params.liquidity_units_eq {
            qb.push(" AND liquidity_units = ")
                .push_bind(liquidity_units_eq);
        }

        if let Some(sort_by) = &params.sort_by {
            qb.push(" ORDER BY ").push(sort_by); // Directly appending column name
        }

        if let Some(order) = &params.order {
            qb.push(" ").push(order.to_lowercase()); // Directly appending order (ASC/DESC)
        }

        //Count
        if let Some(count) = params.count {
            qb.push("LIMIT").push_bind(count);
        }

        // Pagination
        if let Some(limit) = params.limit {
            qb.push(" LIMIT ").push_bind(limit);
        }
        if let Some(page) = params.page {
            let offset = page as i64 * params.limit.unwrap_or(10) as i64;
            qb.push(" OFFSET ").push_bind(offset);
        }

        // ✅ Execute the query
        let query = qb.build();
        println!("SQL Query: {}", query.sql().to_string());
        let result = query.fetch_all(self.pool).await?;

        // ✅ Map result to struct
        Ok(result
            .into_iter()
            .map(|record| PriceHistory {
                id: record.get("id"),
                start_time: record.get("start_time"),
                end_time: record.get("end_time"),
                asset_depth: record.get("asset_depth"),
                rune_depth: record.get("rune_depth"),
                asset_price: record.get("asset_price"),
                asset_price_usd: record.get("asset_price_usd"),
                liquidity_units: record.get("liquidity_units"),
                members_count: record.get("members_count"),
                synth_units: record.get("synth_units"),
                synth_supply: record.get("synth_supply"),
                units: record.get("units"),
                luvi: record.get("luvi"),
            })
            .collect())
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
        .fetch_one(self.pool)
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
