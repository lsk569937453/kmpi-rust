use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteUserReq {
    pub user_id: String,
    pub admin_user_id: String,
}
