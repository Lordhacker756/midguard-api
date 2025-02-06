use sqlx::{Pool, Postgres};
use std::env;
use std::time::Duration;

// src/config/database.rs
use once_cell::sync::OnceCell;

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn initialize_database() -> Result<Pool<Postgres>, sqlx::Error> {
    let url = env::var("DATABASE_URL").map_err(|_| {
        sqlx::Error::Configuration("DATABASE_URL environment variable not set".into())
    })?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await?;

    DB_POOL.set(pool.clone()).unwrap();
    Ok(pool)
}

pub fn get_pool() -> &'static Pool<Postgres> {
    DB_POOL.get().expect("Database pool not initialized")
}

pub async fn run_migrations() -> Result<(), sqlx::Error> {
    let pool = self::get_pool();
    sqlx::migrate!("./migrations").run(pool).await?;

    Ok(())
}
