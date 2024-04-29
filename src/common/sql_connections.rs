use sqlx::{sqlite::SqliteConnectOptions, sqlite::SqlitePool};

pub async fn create_pool() -> Result<SqlitePool, anyhow::Error> {
    let database_url = "sqlite://mydatabase.db";
    let options = SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename(database_url);
    let pool = SqlitePool::connect_with(options).await?;
    Ok(pool)
}
