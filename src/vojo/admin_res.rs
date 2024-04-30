use serde::Serialize;

#[derive(Serialize)]
pub struct AdminLoginRes {
    pub user_id: String,
}
