use axum::{http::StatusCode, routing::get, Router};
use config::database::{self, init_surrealdb};
use dotenv::dotenv;
use error::AppError;
use paris::{info, Logger};
use repositories::{mongodb::MongoPollRepository, surrealdb::SurrealRepository};
use routes::{
    earning_history_route::get_all_earnings_history, price_history_route::get_price_depth_history,
    rune_pool_route::get_all_runepools, swap_history_route::get_all_swap_history,
};
use tower_http::trace::TraceLayer;

mod client;
mod config;
mod cronjobs;
mod dtos;
mod error;
mod model;
mod repositories;
mod routes;
mod service;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    Logger::new();

    // info!("Connecting to postgres database 📔");
    // config::database::initialize_database()
    //     .await
    //     .map_err(|e| AppError::new(e.to_string()).with_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    config::database::init_mongodb()
        .await
        .map_err(|e| AppError::new(e.to_string()).with_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    let mongo_repo = MongoPollRepository::new().await?;
    mongo_repo.populate_mongo_db().await?;
    mongo_repo.read_mongodb().await?;

    // info!("Running database migrations 💿");
    // database::run_migrations()
    //     .await
    //     .map_err(|e| AppError::new(e.to_string()).with_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    // tokio::task::spawn(async {
    //     if let Err(e) = cronjobs::jobs::run().await {
    //         eprintln!("Cronjob error: {}", e);
    //     }
    // });

    init_surrealdb().await?;

    let surreal_repo = SurrealRepository::new().await;
    surreal_repo.insert_into_surreal_db().await;
    surreal_repo.read_from_surreal_db().await;

    let app = Router::new()
        .route("/depth-history", get(get_price_depth_history))
        .route("/earning-history", get(get_all_earnings_history))
        .route("/swap-history", get(get_all_swap_history))
        .route("/runepool-history", get(get_all_runepools))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .map_err(|e| AppError::new(e.to_string()))?;
    println!("Running server at port🌐::{}", 3000);

    axum::serve(listener, app)
        .await
        .map_err(|e| AppError::new(e.to_string()).with_status(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(())
}
