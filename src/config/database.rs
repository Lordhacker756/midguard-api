use crate::error::AppError;
use axum::http::StatusCode;
use sqlx::{Pool, Postgres};
use std::env;
use std::time::Duration;

// src/config/database.rs
use once_cell::sync::OnceCell;

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

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
