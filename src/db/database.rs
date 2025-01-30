use std::env;

pub async fn initialize_database() -> Result<sqlx::PgPool, Box<dyn std::error::Error>> {
    let url = env::var("DATABASE_URL")?;

    let pool = sqlx::postgres::PgPool::connect(&url).await?;
    Ok(pool)
}
