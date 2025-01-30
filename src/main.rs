use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::Row;

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pool = config::database::initialize_database().await?;

    let res = sqlx::query("SELECT 1+1 as sum").fetch_one(&pool).await?;

    let sum: i32 = res.get("sum");
    println!("1 + 1 is {}", sum);

    let app = Router::new().route("/", get(|| async { "Supp ()" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Running server at port🌐::{}", 3000);
    axum::serve(listener, app).await?;

    Ok(())
}
