use sqlx::{query_as, SqlitePool};

#[derive(Debug, sqlx::FromRow)]
struct Vessel {
    id: i32,
    name: String,
    email: String,
}
