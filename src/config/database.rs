use std::env;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub async fn initialize_database() -> Result<Pool<Postgres>, sqlx::Error> {
    let url = env::var("DATABASE_URL").map_err(|_| sqlx::Error::Configuration(
        "DATABASE_URL environment variable not set".into()
    ))?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    Ok(())
}
