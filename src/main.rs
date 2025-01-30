mod constant;
mod model;
use axum::{routing::get, Router};
use dotenv::dotenv;
use std::env;
mod client;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let res = client::proxy::sync_all_data().await.unwrap();

    let _msg = env::var("MSG");
    print!("Enum for Min is {:?}", constant::enums::INTERVALS::Min);
    let app = Router::new().route("/", get(|| async { "Supp ()" }));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running server at port🌐::{}", 3000);
    axum::serve(listener, app).await.unwrap();
}
