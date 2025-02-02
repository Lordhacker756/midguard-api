use axum::{routing::get, Router};
use client::proxy;
use config::database;
use dotenv::dotenv;
use dtos::responses::RunepoolInterval;
use service::{price_history_service::PriceHistoryService, run_pool_service::RunePoolService};

use crate::dtos::responses::PriceDepthInterval;
use model::{price_history::PriceHistory, rune_pool::Runepool};

mod client;
mod config;
mod dtos;
mod model;
mod service;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = config::database::initialize_database().await?;

    database::run_migrations(&pool).await?;

    // // Fetch price history data
    // let intervals: Vec<PriceDepthInterval> = proxy::get_prev_2_months_price_history().await?;

    // // Convert response data to database models
    // let price_histories: Vec<PriceHistory> = intervals.into_iter()
    //     .map(PriceHistory::from)
    //     .collect();

    // // Initialize service and save data
    // let price_history_service = PriceHistoryService::new(pool.clone());
    // let saved_ids = price_history_service.save_batch(&price_histories).await?;
    // println!("Saved {} price history records", saved_ids.len());

    let intervals: Vec<RunepoolInterval> = proxy::get_prev_2_months_runepool_history().await?;

    let rune_pools: Vec<Runepool> = intervals.into_iter().map(Runepool::from).collect();

    let price_history_service = RunePoolService::new(pool.clone());
    let saved_ids = price_history_service.save_batch(&rune_pools).await?;
    println!("Saved {} rune pool records", saved_ids.len());

    print!("{:#?}", rune_pools[0]);

    let app = Router::new().route("/", get(|| async { "Supp ()" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Running server at port🌐::{}", 3000);
    axum::serve(listener, app).await?;

    Ok(())
}
