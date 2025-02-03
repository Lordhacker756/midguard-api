use axum::{routing::get, Router};
use client::proxy;
use config::database;
use dotenv::dotenv;

mod client;
mod config;
mod dtos;
mod model;
mod service;
// mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let pool = config::database::initialize_database().await?;

    database::run_migrations(&pool).await?;

    proxy::sync_all_data(pool.clone()).await?;

    let app = Router::new().route("/", get(|| async { "Supp ()" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Running server at port🌐::{}", 3000);
    axum::serve(listener, app).await?;

    Ok(())
}
