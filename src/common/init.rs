use crate::dao::user::SUPER_ADMIN_AUTHORITY;
use sqlx::sqlite::SqliteRow;
use sqlx::Pool;
use sqlx::Row;
use sqlx::Sqlite;
use tracing::info;
use uuid::Uuid;

pub async fn init_with_error(pool: Pool<Sqlite>) -> Result<(), anyhow::Error> {
    migrate(pool.clone()).await?;
    init_super_user(pool).await?;
    Ok(())
}
async fn migrate(pool: Pool<Sqlite>) -> Result<(), anyhow::Error> {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| anyhow!("{}", e))
}
async fn init_super_user(pool: Pool<Sqlite>) -> Result<(), sqlx::Error> {
    let count = sqlx::query("SELECT COUNT(*) FROM user WHERE user_authority = $1")
        .bind(SUPER_ADMIN_AUTHORITY)
        .map(|row: SqliteRow| row.try_get::<i64, _>(0))
        .fetch_one(&pool)
        .await??;

    let uuid = Uuid::new_v4().to_string();
    if count == 0 {
        sqlx::query("INSERT INTO user (user_account, user_password, user_authority, user_id) VALUES ($1, $2, $3, $4)")
                .bind("admin".to_string())
                .bind("zc12345679".to_string())
                .bind(SUPER_ADMIN_AUTHORITY)
                .bind(uuid)
                .execute(&pool)
                .await?;
        info!("Super User has been created");
    } else {
        info!("Super User Exists");
    }
    Ok(())
}
