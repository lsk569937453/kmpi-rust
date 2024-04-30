use serde::Deserialize;

#[derive(Deserialize)]
pub struct AdminLoginReq {
    pub admin_account: String,
    pub admin_password: String,
}
