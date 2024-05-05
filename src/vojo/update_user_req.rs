use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateUserReq {
    pub user_account: String,
    pub user_password: String,
    pub admin_user_id: String,
    pub user_id: String,
    pub user_authority: u32,
}
