use serde::Serialize;
#[derive(Serialize)]

pub struct LoginRes {
    pub user_id: String,
    pub authority: u32,
}
