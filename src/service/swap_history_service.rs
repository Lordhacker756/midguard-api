use crate::{
    config::database::get_pool,
    model::swap_history::{QueryParams, SwapHistory},
};
use anyhow::{Error, Result};
use axum::extract::Query;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder};

pub struct SwapHistoryService<'a> {
    pool: &'a PgPool,
}

impl<'a> SwapHistoryService<'a> {
    pub fn new() -> Self {
        Self { pool: get_pool() }
    }

    pub async fn get_last_update_timestamp(&self) -> Result<i64, Error> {
        let record =
            sqlx::query!("SELECT start_time FROM swap_history ORDER BY start_time DESC LIMIT 1")
                .fetch_one(self.pool)
                .await?;

        Ok(record.start_time.timestamp())
    }

    // Fix the generic type constraints for the comparison filters method
    fn add_comparison_filters<'q, T>(
        &self,
        qb: &mut QueryBuilder<'q, Postgres>,
        field: &str,
        gt: Option<T>,
        lt: Option<T>,
        eq: Option<T>,
    ) where
        T: sqlx::Encode<'q, Postgres> + sqlx::Type<Postgres> + std::fmt::Display + Send + 'q,
    {
        if let Some(val) = gt {
            qb.push(format!(" AND {} > ", field)).push_bind(val);
        }
        if let Some(val) = lt {
            qb.push(format!(" AND {} < ", field)).push_bind(val);
        }
        if let Some(val) = eq {
            qb.push(format!(" AND {} = ", field)).push_bind(val);
        }
    }

    pub async fn get_all_swap_history(
        &self,
        params: Query<QueryParams>,
    ) -> Result<Vec<SwapHistory>, Error> {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM swap_history WHERE true");

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
                _ => "date_trunc('hour', start_time)",
            };

            qb.push(" AND start_time IN (");
            qb.push("SELECT DISTINCT ON (")
                .push(interval_trunc)
                .push(") start_time FROM swap_history WHERE true");
            qb.push(")");
        }

        // Date range filter
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

        // Add all comparison filters
        self.add_comparison_filters(
            &mut qb,
            "from_trade_average_slip",
            params.from_trade_average_slip_gt,
            params.from_trade_average_slip_lt,
            params.from_trade_average_slip_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "from_trade_count",
            params.from_trade_count_gt,
            params.from_trade_count_lt,
            params.from_trade_count_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "from_trade_fees",
            params.from_trade_fees_gt,
            params.from_trade_fees_lt,
            params.from_trade_fees_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "from_trade_volume",
            params.from_trade_volume_gt,
            params.from_trade_volume_lt,
            params.from_trade_volume_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "from_trade_volume_usd",
            params.from_trade_volume_usd_gt,
            params.from_trade_volume_usd_lt,
            params.from_trade_volume_usd_eq,
        );

        // Synth mint filters
        self.add_comparison_filters(
            &mut qb,
            "synth_mint_average_slip",
            params.synth_mint_average_slip_gt,
            params.synth_mint_average_slip_lt,
            params.synth_mint_average_slip_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "synth_mint_count",
            params.synth_mint_count_gt,
            params.synth_mint_count_lt,
            params.synth_mint_count_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "synth_mint_fees",
            params.synth_mint_fees_gt,
            params.synth_mint_fees_lt,
            params.synth_mint_fees_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "synth_mint_volume",
            params.synth_mint_volume_gt,
            params.synth_mint_volume_lt,
            params.synth_mint_volume_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "synth_mint_volume_usd",
            params.synth_mint_volume_usd_gt,
            params.synth_mint_volume_usd_lt,
            params.synth_mint_volume_usd_eq,
        );

        // Synth redeem filters
        self.add_comparison_filters(
            &mut qb,
            "synth_redeem_average_slip",
            params.synth_redeem_average_slip_gt,
            params.synth_redeem_average_slip_lt,
            params.synth_redeem_average_slip_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "synth_redeem_count",
            params.synth_redeem_count_gt,
            params.synth_redeem_count_lt,
            params.synth_redeem_count_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "synth_redeem_fees",
            params.synth_redeem_fees_gt,
            params.synth_redeem_fees_lt,
            params.synth_redeem_fees_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "synth_redeem_volume",
            params.synth_redeem_volume_gt,
            params.synth_redeem_volume_lt,
            params.synth_redeem_volume_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "synth_redeem_volume_usd",
            params.synth_redeem_volume_usd_gt,
            params.synth_redeem_volume_usd_lt,
            params.synth_redeem_volume_usd_eq,
        );

        // To asset filters
        self.add_comparison_filters(
            &mut qb,
            "to_asset_average_slip",
            params.to_asset_average_slip_gt,
            params.to_asset_average_slip_lt,
            params.to_asset_average_slip_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_asset_count",
            params.to_asset_count_gt,
            params.to_asset_count_lt,
            params.to_asset_count_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_asset_fees",
            params.to_asset_fees_gt,
            params.to_asset_fees_lt,
            params.to_asset_fees_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_asset_volume",
            params.to_asset_volume_gt,
            params.to_asset_volume_lt,
            params.to_asset_volume_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_asset_volume_usd",
            params.to_asset_volume_usd_gt,
            params.to_asset_volume_usd_lt,
            params.to_asset_volume_usd_eq,
        );

        // To rune filters
        self.add_comparison_filters(
            &mut qb,
            "to_rune_average_slip",
            params.to_rune_average_slip_gt,
            params.to_rune_average_slip_lt,
            params.to_rune_average_slip_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_rune_count",
            params.to_rune_count_gt,
            params.to_rune_count_lt,
            params.to_rune_count_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_rune_fees",
            params.to_rune_fees_gt,
            params.to_rune_fees_lt,
            params.to_rune_fees_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_rune_volume",
            params.to_rune_volume_gt,
            params.to_rune_volume_lt,
            params.to_rune_volume_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_rune_volume_usd",
            params.to_rune_volume_usd_gt,
            params.to_rune_volume_usd_lt,
            params.to_rune_volume_usd_eq,
        );

        // To trade filters
        self.add_comparison_filters(
            &mut qb,
            "to_trade_average_slip",
            params.to_trade_average_slip_gt,
            params.to_trade_average_slip_lt,
            params.to_trade_average_slip_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_trade_count",
            params.to_trade_count_gt,
            params.to_trade_count_lt,
            params.to_trade_count_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_trade_fees",
            params.to_trade_fees_gt,
            params.to_trade_fees_lt,
            params.to_trade_fees_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_trade_volume",
            params.to_trade_volume_gt,
            params.to_trade_volume_lt,
            params.to_trade_volume_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "to_trade_volume_usd",
            params.to_trade_volume_usd_gt,
            params.to_trade_volume_usd_lt,
            params.to_trade_volume_usd_eq,
        );

        // Total metrics filters
        self.add_comparison_filters(
            &mut qb,
            "total_count",
            params.total_count_gt,
            params.total_count_lt,
            params.total_count_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "total_fees",
            params.total_fees_gt,
            params.total_fees_lt,
            params.total_fees_eq,
        );

        self.add_comparison_filters(
            &mut qb,
            "total_volume",
            params.total_volume_gt,
            params.total_volume_lt,
            params.total_volume_eq,
        );

        // Sorting
        if let Some(sort_by) = &params.sort_by {
            qb.push(" ORDER BY ").push(sort_by);

            if let Some(order) = &params.order {
                match order.to_lowercase().as_str() {
                    "asc" => qb.push(" ASC"),
                    "desc" => qb.push(" DESC"),
                    _ => qb.push(" ASC"),
                };
            }
        }

        // Pagination
        if let Some(count) = params.count {
            qb.push(" LIMIT ").push_bind(count);
        } else if let Some(limit) = params.limit {
            qb.push(" LIMIT ").push_bind(limit);

            if let Some(page) = params.page {
                let offset = (page as i64).saturating_sub(1) * limit as i64;
                qb.push(" OFFSET ").push_bind(offset);
            }
        }

        let query = qb.build_query_as::<SwapHistory>();
        println!("SQL Query: {}", query.sql());
        let result = query.fetch_all(self.pool).await?;

        Ok(result)
    }

    pub async fn save(&self, swap_history: &SwapHistory) -> Result<i32> {
        let result = sqlx::query!(r#"
                INSERT INTO swap_history (
                            average_slip, end_time, from_trade_average_slip, from_trade_count, from_trade_fees, from_trade_volume, from_trade_volume_usd,
                            rune_price_usd, start_time, synth_mint_average_slip, synth_mint_count, synth_mint_fees, synth_mint_volume, synth_mint_volume_usd,
                            synth_redeem_average_slip, synth_redeem_count, synth_redeem_fees, synth_redeem_volume, synth_redeem_volume_usd,
                            to_asset_average_slip, to_asset_count, to_asset_fees, to_asset_volume, to_asset_volume_usd,
                            to_rune_average_slip, to_rune_count, to_rune_fees, to_rune_volume, to_rune_volume_usd,
                            to_trade_average_slip, to_trade_count, to_trade_fees, to_trade_volume, to_trade_volume_usd,
                            total_count, total_fees, total_volume, total_volume_usd
                            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, 
                            $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37, $38
                )
                RETURNING id
            "#,
            swap_history.average_slip,
            swap_history.end_time,
            swap_history.from_trade_average_slip,
            swap_history.from_trade_count,
            swap_history.from_trade_fees,
            swap_history.from_trade_volume,
            swap_history.from_trade_volume_usd,
            swap_history.rune_price_usd,
            swap_history.start_time,
            swap_history.synth_mint_average_slip,
            swap_history.synth_mint_count,
            swap_history.synth_mint_fees,
            swap_history.synth_mint_volume,
            swap_history.synth_mint_volume_usd,
            swap_history.synth_redeem_average_slip,
            swap_history.synth_redeem_count,
            swap_history.synth_redeem_fees,
            swap_history.synth_redeem_volume,
            swap_history.synth_redeem_volume_usd,
            swap_history.to_asset_average_slip,
            swap_history.to_asset_count,
            swap_history.to_asset_fees,
            swap_history.to_asset_volume,
            swap_history.to_asset_volume_usd,
            swap_history.to_rune_average_slip,
            swap_history.to_rune_count,
            swap_history.to_rune_fees,
            swap_history.to_rune_volume,
            swap_history.to_rune_volume_usd,
            swap_history.to_trade_average_slip,
            swap_history.to_trade_count,
            swap_history.to_trade_fees,
            swap_history.to_trade_volume,
            swap_history.to_trade_volume_usd,
            swap_history.total_count,
            swap_history.total_fees,
            swap_history.total_volume,
            swap_history.total_volume_usd)
            .fetch_one(self.pool)
            .await?;

        Ok(result.id)
    }

    pub async fn save_batch(&self, swap_histories: &[SwapHistory]) -> Result<Vec<i32>> {
        println!(
            "📦 Batch saving {} swap history records",
            swap_histories.len()
        );
        const BATCH_SIZE: usize = 1000;
        let mut results = Vec::new();

        for chunk in swap_histories.chunks(BATCH_SIZE) {
            let mut tx = self.pool.begin().await?;

            for record in chunk {
                let id = sqlx::query!(
                    r#"
                    INSERT INTO swap_history (
                        average_slip, end_time, from_trade_average_slip, from_trade_count,
                        from_trade_fees, from_trade_volume, from_trade_volume_usd, rune_price_usd,
                        start_time, synth_mint_average_slip, synth_mint_count, synth_mint_fees,
                        synth_mint_volume, synth_mint_volume_usd, synth_redeem_average_slip,
                        synth_redeem_count, synth_redeem_fees, synth_redeem_volume,
                        synth_redeem_volume_usd, to_asset_average_slip, to_asset_count,
                        to_asset_fees, to_asset_volume, to_asset_volume_usd, to_rune_average_slip,
                        to_rune_count, to_rune_fees, to_rune_volume, to_rune_volume_usd,
                        to_trade_average_slip, to_trade_count, to_trade_fees, to_trade_volume,
                        to_trade_volume_usd, total_count, total_fees, total_volume, total_volume_usd
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, 
                            $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, 
                            $29, $30, $31, $32, $33, $34, $35, $36, $37, $38)
                    RETURNING id"#,
                    record.average_slip,
                    record.end_time,
                    record.from_trade_average_slip,
                    record.from_trade_count,
                    record.from_trade_fees,
                    record.from_trade_volume,
                    record.from_trade_volume_usd,
                    record.rune_price_usd,
                    record.start_time,
                    record.synth_mint_average_slip,
                    record.synth_mint_count,
                    record.synth_mint_fees,
                    record.synth_mint_volume,
                    record.synth_mint_volume_usd,
                    record.synth_redeem_average_slip,
                    record.synth_redeem_count,
                    record.synth_redeem_fees,
                    record.synth_redeem_volume,
                    record.synth_redeem_volume_usd,
                    record.to_asset_average_slip,
                    record.to_asset_count,
                    record.to_asset_fees,
                    record.to_asset_volume,
                    record.to_asset_volume_usd,
                    record.to_rune_average_slip,
                    record.to_rune_count,
                    record.to_rune_fees,
                    record.to_rune_volume,
                    record.to_rune_volume_usd,
                    record.to_trade_average_slip,
                    record.to_trade_count,
                    record.to_trade_fees,
                    record.to_trade_volume,
                    record.to_trade_volume_usd,
                    record.total_count,
                    record.total_fees,
                    record.total_volume,
                    record.total_volume_usd
                )
                .fetch_one(&mut *tx)
                .await?;

                results.push(id.id);
            }

            tx.commit().await?;
        }

        Ok(results)
    }
}
