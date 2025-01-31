use axum::{routing::get, Router};
use client::proxy;
use config::database;
use dotenv::dotenv;
use model::{
    earning_history::EarningHistory,
    price_history::PriceHistory,
    responses::{EarningInterval, Interval},
};
use sqlx::Row;

mod client;
mod config;
mod model;
mod repository;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pool = config::database::initialize_database().await?;

    database::run_migrations(&pool).await?;
    let res = sqlx::query("SELECT 1+1 as sum").fetch_one(&pool).await?;

    let sum: i32 = res.get("sum");
    println!("1 + 1 is {}", sum);

    let res: Vec<EarningInterval> = proxy::get_prev_2_months_earning_history().await?;

    let converted: EarningHistory = EarningHistory::try_from(res[0].clone())?;

    print!("{:?}", converted);

    let app = Router::new().route("/", get(|| async { "Supp ()" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Running server at port🌐::{}", 3000);
    axum::serve(listener, app).await?;

    Ok(())
}
