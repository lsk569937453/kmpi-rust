use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginReq {
    pub name: String,
    pub password: String,
}
