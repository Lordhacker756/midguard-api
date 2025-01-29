use axum::{routing::get, Router};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let _msg = env::var("MSG");
    let app = Router::new().route("/", get(|| async { "Supp ()" }));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running server at port🌐::{}", 3000);
    axum::serve(listener, app).await.unwrap();
}
