use sqlx::{sqlite::SqliteConnectOptions, sqlite::SqlitePool};
use std::env;
use std::str::FromStr;
pub async fn create_pool() -> Result<SqlitePool, anyhow::Error> {
    env::set_var("DATABASE_URL", "sqlite://mydatabase.db");
    let database_url = env::var("DATABASE_URL").expect("database_url is not exist");
    let options = SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;
    Ok(pool)
}
