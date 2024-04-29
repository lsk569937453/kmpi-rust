use sqlx::{query_as, SqlitePool};
pub const SUPER_ADMIN_AUTHORITY: i32 = 100;
pub const ADMIN_AUTHORITY: i32 = 99;
pub const UPDATE_AUTHORITY: i32 = 98;
pub const SELECT_AUTHORITY: i32 = 97;
#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    user_id: String,
    user_account: String,
    user_authority: String,
    timestamp: String,
}
