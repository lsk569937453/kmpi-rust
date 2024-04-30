use crate::dao::user::User;
use crate::dao::user::SUPER_ADMIN_AUTHORITY;
use crate::vojo::admin_req::AdminLoginReq;
use crate::vojo::admin_res::AdminLoginRes;
use crate::vojo::base_response::to_string;
use crate::vojo::base_response::BaseResponse;
use crate::vojo::login_req::LoginReq;
use crate::vojo::login_res::LoginRes;
use axum::extract::Form;
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use sqlx::{Pool, Sqlite};
use std::convert::Infallible;
use tracing::info;

pub async fn admin_login(
    State(state): State<Pool<Sqlite>>,
    Json(req): Json<AdminLoginReq>,
) -> Result<Response, Infallible> {
    match admin_login_with_error(state, req).await {
        Ok(r) => Ok((
            axum::http::StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            r,
        )
            .into_response()),
        Err(e) => {
            Ok((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())
        }
    }
}
async fn admin_login_with_error(
    pool: Pool<Sqlite>,
    req: AdminLoginReq,
) -> Result<String, anyhow::Error> {
    let query = format!(
        r#"SELECT * FROM user WHERE user_account = "{}" AND user_password = "{}""#,
        req.admin_account, req.admin_password
    );
    let res = sqlx::query_as::<Sqlite, User>(&query)
        .fetch_one(&pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => anyhow!("user or password error"),
            _ => anyhow!("query error,the error is {}", e),
        })?;

    if res.user_authority != SUPER_ADMIN_AUTHORITY {
        return Err(anyhow!("not admin"));
    }

    let admin_res = AdminLoginRes {
        user_id: res.user_id,
    };

    let data = BaseResponse {
        response_code: 0,
        response_object: admin_res,
    };
    serde_json::to_string(&data).map_err(|e| anyhow!("{}", e))
}
pub async fn login(
    State(state): State<Pool<Sqlite>>,
    Json(req): Json<LoginReq>,
) -> Result<Response, Infallible> {
    match login_with_error(state, req).await {
        Ok(r) => Ok((
            axum::http::StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json")],
            r,
        )
            .into_response()),
        Err(e) => {
            Ok((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response())
        }
    }
}
async fn login_with_error(pool: Pool<Sqlite>, req: LoginReq) -> Result<String, anyhow::Error> {
    let query = format!(
        r#"SELECT * FROM user WHERE user_account = "{}" AND user_password = "{}""#,
        req.name, req.password
    );
    let res = sqlx::query_as::<Sqlite, User>(&query)
        .fetch_optional(&pool)
        .await
        .map_err(|e| anyhow!("query error,the error is {}", e))?;

    match res {
        Some(user) => {
            let login_res = LoginRes {
                user_id: user.user_id,
                authority: user.user_authority,
            };
            to_string(0, login_res)
        }
        None => to_string(-1, String::from("User or password are not correct!")),
    }
}
