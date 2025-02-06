use anyhow::{Error, Result};
use axum::extract::Query;
use sqlx::{Execute, PgPool, Postgres, QueryBuilder, Row};

use crate::{
    config::database::get_pool,
    model::{
        earning_history::{EarningHistory, QueryParams},
        earning_history_pool::{EarningHistoryPool, EarningPoolQueryParams},
    },
};

pub struct EarningHistoryService<'a> {
    pool: &'a PgPool,
}

impl<'a> EarningHistoryService<'a> {
    pub fn new() -> Self {
        Self { pool: get_pool() }
    }

    pub async fn get_all_pools(
        &self,
        earning_history_id: i32,
        params: &Query<QueryParams>,
    ) -> Result<Vec<EarningHistoryPool>> {
        let mut qb = QueryBuilder::<Postgres>::new(
            "SELECT * FROM pool_earnings WHERE earnings_history_id = ",
        );
        qb.push(earning_history_id);

        // Pool filter
        if let Some(pool) = &params.pool_eq {
            qb.push(" AND pool = ").push_bind(pool);
        }

        // Asset liquidity fees filters
        if let Some(lt) = params.pool_asset_liquidity_fees_lt {
            qb.push(" AND asset_liquidity_fees < ").push_bind(lt);
        }
        if let Some(eq) = params.pool_asset_liquidity_fees_eq {
            qb.push(" AND asset_liquidity_fees = ").push_bind(eq);
        }
        if let Some(gt) = params.pool_asset_liquidity_fees_gt {
            qb.push(" AND asset_liquidity_fees > ").push_bind(gt);
        }

        // Rune liquidity fees filters
        if let Some(lt) = params.pool_rune_liquidity_fees_lt {
            qb.push(" AND rune_liquidity_fees < ").push_bind(lt);
        }
        if let Some(eq) = params.pool_rune_liquidity_fees_eq {
            qb.push(" AND rune_liquidity_fees = ").push_bind(eq);
        }
        if let Some(gt) = params.pool_rune_liquidity_fees_gt {
            qb.push(" AND rune_liquidity_fees > ").push_bind(gt);
        }

        // Total liquidity fees filters
        if let Some(lt) = params.pool_total_liquidity_fees_lt {
            qb.push(" AND total_liquidity_fees_rune < ").push_bind(lt);
        }
        if let Some(eq) = params.pool_total_liquidity_fees_eq {
            qb.push(" AND total_liquidity_fees_rune = ").push_bind(eq);
        }
        if let Some(gt) = params.pool_total_liquidity_fees_gt {
            qb.push(" AND total_liquidity_fees_rune > ").push_bind(gt);
        }

        // Saver earning filters
        if let Some(lt) = params.pool_saver_earning_lt {
            qb.push(" AND saver_earning < ").push_bind(lt);
        }
        if let Some(eq) = params.pool_saver_earning_eq {
            qb.push(" AND saver_earning = ").push_bind(eq);
        }
        if let Some(gt) = params.pool_saver_earning_gt {
            qb.push(" AND saver_earning > ").push_bind(gt);
        }

        // Pool rewards filters
        if let Some(lt) = params.pool_rewards_lt {
            qb.push(" AND rewards < ").push_bind(lt);
        }
        if let Some(eq) = params.pool_rewards_eq {
            qb.push(" AND rewards = ").push_bind(eq);
        }
        if let Some(gt) = params.pool_rewards_gt {
            qb.push(" AND rewards > ").push_bind(gt);
        }

        // Pool earnings filters
        if let Some(lt) = params.pool_earnings_lt {
            qb.push(" AND earnings < ").push_bind(lt);
        }
        if let Some(eq) = params.pool_earnings_eq {
            qb.push(" AND earnings = ").push_bind(eq);
        }
        if let Some(gt) = params.pool_earnings_gt {
            qb.push(" AND earnings > ").push_bind(gt);
        }

        let query = qb.build();
        println!("SQL Query: {}", query.sql().to_string());
        let result = query.fetch_all(self.pool).await?;

        Ok(result
            .into_iter()
            .map(|record| EarningHistoryPool {
                id: Some(record.get("id")),
                earnings_history_id: Some(record.get("earnings_history_id")),
                pool: record.get("pool"),
                asset_liquidity_fees: record.get("asset_liquidity_fees"),
                rune_liquidity_fees: record.get("rune_liquidity_fees"),
                total_liquidity_fees_rune: record.get("total_liquidity_fees_rune"),
                saver_earning: record.get("saver_earning"),
                rewards: record.get("rewards"),
                earnings: record.get("earnings"),
            })
            .collect())
    }

    pub async fn get_all_earnings_history(
        &self,
        params: Query<QueryParams>,
    ) -> Result<Vec<EarningHistory>, Error> {
        let mut qb = QueryBuilder::<Postgres>::new("SELECT * FROM earnings_history WHERE true");
        if let Some(interval) = &params.interval {
            let interval_code = match interval.as_str() {
                        "5min" => "dat ̰e_trunc('minute', start_time) + INTERVAL '5 minutes' * (EXTRACT(MINUTE FROM start_time)::int / 5)",
        "hour" => "date_trunc('hour', start_time)",
        "day" => "date_trunc('day', start_time)",
        "week" => "date_trunc('week', start_time)",
        "month" => "date_trunc('month', start_time)",
        "quarter" => "date_trunc('quarter', start_time)",
        "year" => "date_trunc('year', start_time)",
        _ => "date_trunc('hour', start_time)", // Default to hourly
            };

            qb.push(" AND start_time in (");
            qb.push("SELECT DISCTINCT ON (")
                .push(interval_code)
                .push(") start_time FROM earnings_history");
            qb.push(" WHERE true ");
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

        // Remaining query params filters
        // Liquidity fees
        if let Some(lt) = params.liquidity_fees_lt {
            qb.push(" AND liquidity_fees < ").push_bind(lt);
        }
        if let Some(eq) = params.liquidity_fees_eq {
            qb.push(" AND liquidity_fees = ").push_bind(eq);
        }
        if let Some(gt) = params.liquidity_fees_gt {
            qb.push(" AND liquidity_fees > ").push_bind(gt);
        }

        // Block rewards
        if let Some(lt) = params.block_rewards_lt {
            qb.push(" AND block_rewards < ").push_bind(lt);
        }
        if let Some(eq) = params.block_rewards_eq {
            qb.push(" AND block_rewards = ").push_bind(eq);
        }
        if let Some(gt) = params.block_rewards_gt {
            qb.push(" AND block_rewards > ").push_bind(gt);
        }

        // Earnings
        if let Some(lt) = params.earnings_lt {
            qb.push(" AND earnings < ").push_bind(lt);
        }
        if let Some(eq) = params.earnings_eq {
            qb.push(" AND earnings = ").push_bind(eq);
        }
        if let Some(gt) = params.earnings_gt {
            qb.push(" AND earnings > ").push_bind(gt);
        }

        // Bonding earnings
        if let Some(lt) = params.bonding_earnings_lt {
            qb.push(" AND bonding_earnings < ").push_bind(lt);
        }
        if let Some(eq) = params.bonding_earnings_eq {
            qb.push(" AND bonding_earnings = ").push_bind(eq);
        }
        if let Some(gt) = params.bonding_earnings_gt {
            qb.push(" AND bonding_earnings > ").push_bind(gt);
        }

        // Liquidity earnings
        if let Some(lt) = params.liquidity_earnings_lt {
            qb.push(" AND liquidity_earnings < ").push_bind(lt);
        }
        if let Some(eq) = params.liquidity_earnings_eq {
            qb.push(" AND liquidity_earnings = ").push_bind(eq);
        }
        if let Some(gt) = params.liquidity_earnings_gt {
            qb.push(" AND liquidity_earnings > ").push_bind(gt);
        }

        // Avg node count
        if let Some(lt) = params.avg_node_count_lt {
            qb.push(" AND avg_node_count < ").push_bind(lt);
        }
        if let Some(eq) = params.avg_node_count_eq {
            qb.push(" AND avg_node_count = ").push_bind(eq);
        }
        if let Some(gt) = params.avg_node_count_gt {
            qb.push(" AND avg_node_count > ").push_bind(gt);
        }

        // Rune price USD
        if let Some(lt) = params.rune_price_usd_lt {
            qb.push(" AND rune_price_usd < ").push_bind(lt);
        }
        if let Some(eq) = params.rune_price_usd_eq {
            qb.push(" AND rune_price_usd = ").push_bind(eq);
        }
        if let Some(gt) = params.rune_price_usd_gt {
            qb.push(" AND rune_price_usd > ").push_bind(gt);
        }

        // Sort order
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

        let query = qb.build();
        println!("SQL Query: {}", query.sql().to_string());
        let result = query.fetch_all(self.pool).await?;

        let mut earnings = result
            .into_iter()
            .map(|record| EarningHistory {
                id: Some(record.get("id")),
                start_time: record.get("start_time"),
                end_time: record.get("end_time"),
                liquidity_fees: record.get("liquidity_fees"),
                block_rewards: record.get("block_rewards"),
                earnings: record.get("earnings"),
                bonding_earnings: record.get("bonding_earnings"),
                liquidity_earnings: record.get("liquidity_earnings"),
                avg_node_count: record.get("avg_node_count"),
                rune_price_usd: record.get("rune_price_usd"),
                pools: None,
            })
            .collect::<Vec<EarningHistory>>();

        for earning in &mut earnings {
            let pools = self.get_all_pools(earning.id.unwrap(), &params).await?;
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
