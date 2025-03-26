use crate::error::AppError;
use axum::http::StatusCode;
use mongodb::{Client, Database};
use paris::{error, info, success};
use sqlx::{Pool, Postgres};
use std::env;
use std::sync::Arc;
use std::sync::LazyLock;
use std::time::Duration;
use surrealdb::engine::remote::ws::Client as SurrealClient;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

// src/config/database.rs
use once_cell::sync::OnceCell;

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
static MONGO_POOL: OnceCell<Database> = OnceCell::new();
static SURREAL_DB: LazyLock<Surreal<SurrealClient>> = LazyLock::new(Surreal::init);

pub async fn initialize_database() -> Result<Pool<Postgres>, AppError> {
    let url = env::var("DATABASE_URL").map_err(|_| {
        AppError::new("DATABASE_URL environment variable not set")
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await
        .map_err(|e| AppError::new(format!("Failed to connect to database: {}", e)))?;

    DB_POOL.set(pool.clone()).map_err(|_| {
        AppError::new("Failed to set database pool").with_status(StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    success!("Connected to PostGres DB");

    Ok(pool)
}

pub fn get_pool() -> Result<&'static Pool<Postgres>, AppError> {
    DB_POOL.get().ok_or_else(|| {
        AppError::new("Database pool not initialized")
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
    })
}

pub async fn run_migrations() -> Result<(), AppError> {
    let pool = self::get_pool()?;
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::new(format!("Migration failed: {}", e)))?;

    Ok(())
}

//* Mongo DB */
pub async fn init_mongodb() -> mongodb::error::Result<Arc<Database>> {
    info!("Connecting to mongodb🍀");
    let mongo_uri = env::var("MONGO_URI").map_err(|_| {
        error!("MONGO_URI not found in environment variables");
        mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "MONGO_URI not found",
        ))
    })?;

    let db_name = env::var("DATABASE_NAME").map_err(|_| {
        error!("DATABASE_NAME not found in environment variables");
        mongodb::error::Error::from(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "DATABASE_NAME not found",
        ))
    })?;

    let client = Client::with_uri_str(&mongo_uri).await?;
    let database = client.database(&db_name);

    let _ = MONGO_POOL
        .set(database.clone())
        .map_err(|_| AppError::new("Error connecting to mongodb"));

    success!("Connected to MongoDB");

    Ok(Arc::new(database))
}

pub async fn get_mongo_pool() -> Result<&'static Database, AppError> {
    MONGO_POOL.get().ok_or_else(|| {
        AppError::new("Database pool not initialized")
            .with_status(StatusCode::INTERNAL_SERVER_ERROR)
    })
}

// Surreal DB */
pub async fn init_surrealdb() -> Result<&'static Surreal<SurrealClient>, AppError> {
    info!("Connecting to SurrealDB...");

    let endpoint = env::var("SURREAL_ENDPOINT").unwrap_or_else(|_| "localhost:80".to_string());
    let username = env::var("SURREAL_USER").unwrap_or_else(|_| "root".to_string());
    let password = env::var("SURREAL_PASS").unwrap_or_else(|_| "root".to_string());
    let namespace = env::var("SURREAL_NS").unwrap_or_else(|_| "test".to_string());
    let database = env::var("SURREAL_DB").unwrap_or_else(|_| "test".to_string());

    SURREAL_DB
        .connect::<Ws>(endpoint)
        .await
        .map_err(|e| AppError::new(format!("Failed to connect to SurrealDB: {}", e)))?;

    SURREAL_DB
        .signin(Root {
            username: &username,
            password: &password,
        })
        .await
        .map_err(|e| AppError::new(format!("Failed to authenticate with SurrealDB: {}", e)))?;

    SURREAL_DB
        .use_ns(namespace)
        .use_db(database)
        .await
        .map_err(|e| AppError::new(format!("Failed to use namespace and database: {}", e)))?;

    success!("Connected to SurrealDB");
    Ok(&SURREAL_DB)
}

pub async fn get_surreal_db() -> &'static Surreal<SurrealClient> {
    return &SURREAL_DB;
}
