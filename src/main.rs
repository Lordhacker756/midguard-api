use axum::{routing::get, Router};
use config::database;
use dotenv::dotenv;
use routes::{
    earning_history_route::get_all_earnings_history, price_history_route::get_price_depth_history,
};

mod client;
mod config;
mod dtos;
mod model;
mod routes;
mod service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    config::database::initialize_database().await?;

    database::run_migrations().await?;

    // proxy::sync_all_data(pool.clone()).await?;

    let app = Router::new()
        .route("/depth-history", get(get_price_depth_history))
        .route("/earning-history", get(get_all_earnings_history));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Running server at port🌐::{}", 3000);
    axum::serve(listener, app).await?;

    Ok(())
}
