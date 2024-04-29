use sqlx::{sqlite::SqliteConnectOptions, sqlite::SqlitePool};
use std::str::FromStr;
pub async fn create_pool() -> Result<SqlitePool, anyhow::Error> {
    let options = SqliteConnectOptions::from_str("sqlite://mydatabase.db")?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    Ok(pool)
}
