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
        println!("📊 Initializing PriceHistoryService");
        Self { pool: get_pool() }
    }

    pub async fn get_last_update_timestamp(&self) -> Result<i64, Error> {
        let record = sqlx::query!(
            "SELECT start_time FROM depth_price_history ORDER BY start_time DESC LIMIT 1"
        )
        .fetch_one(self.pool)
        .await?;

        Ok(record.start_time.timestamp())
    }

    pub async fn get_all_price_history(
        &self,
        params: Query<PriceHistoryParams>,
    ) -> Result<Vec<PriceHistory>, Error> {
        println!("🔍 Fetching price history with params: {:?}", params);
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM depth_price_history WHERE true");
        // Interval filter
        if let Some(interval) = &params.interval {
            println!("⏱️ Applying interval filter: {}", interval);
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
            println!("📅 Applying date range filter: {}", date_range);
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
            println!("💰 Applying asset depth filter: > {}", asset_depth_gt);
            qb.push(" AND asset_depth > ").push_bind(asset_depth_gt);
        }
        if let Some(asset_depth_lt) = params.asset_depth_lt {
            qb.push(" AND asset_depth < ").push_bind(asset_depth_lt);
        }
        if let Some(asset_depth_eq) = params.asset_depth_eq {
            qb.push(" AND asset_depth = ").push_bind(asset_depth_eq);
        }

        // [Rest of the filters remain the same...]

        // ✅ Execute the query
        let query = qb.build();
        println!("🔎 Executing SQL Query: {}", query.sql());
        let result = query.fetch_all(self.pool).await?;
        println!("✅ Found {} records", result.len());

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
        println!("💾 Saving single price history record");
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

        println!("✅ Saved record with ID: {}", result.id);
        Ok(result.id)
    }

    pub async fn save_batch(&self, price_histories: &[PriceHistory]) -> Result<Vec<i32>> {
        println!(
            "📦 Batch saving {} price history records",
            price_histories.len()
        );

        let mut tx = self.pool.begin().await?;

        let copy = String::from(
            "COPY depth_price_history (start_time, end_time, asset_depth, rune_depth, \
         asset_price, asset_price_usd, liquidity_units, members_count, synth_units, \
         synth_supply, units, luvi) FROM STDIN WITH (FORMAT text, DELIMITER '\t')",
        );

        let mut writer = tx.copy_in_raw(&copy).await?;

        // Process in chunks of 5000 records
        for chunk in price_histories.chunks(5000) {
            let mut batch_data = String::with_capacity(chunk.len() * 256);

            for price_history in chunk {
                batch_data.push_str(&format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
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
                    price_history.luvi,
                ));
            }

            writer.send(batch_data.as_bytes()).await?;
        }

        writer.finish().await?;

        let ids = sqlx::query_as::<_, (i32,)>(
            "SELECT id FROM depth_price_history ORDER BY id DESC LIMIT $1",
        )
        .bind(price_histories.len() as i32)
        .fetch_all(&mut *tx)
        .await?
        .into_iter()
        .map(|(id,)| id)
        .collect();

        tx.commit().await?;

        println!("🎉 Successfully saved {} records", price_histories.len());
        Ok(ids)
    }
}
