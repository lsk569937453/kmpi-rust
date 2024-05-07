use crate::dao::user::User;
use crate::dao::user::SUPER_ADMIN_AUTHORITY;
use crate::vojo::add_user_req::AddUserReq;
use crate::vojo::admin_req::AdminLoginReq;
use crate::vojo::admin_res::AdminLoginRes;
use crate::vojo::base_response::to_string;
use crate::vojo::base_response::BaseResponse;
use crate::vojo::delete_user_req::DeleteUserReq;
use crate::vojo::login_req::LoginReq;
use crate::vojo::login_res::LoginRes;
use crate::vojo::update_user_req::UpdateUserReq;
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use sqlx::{Pool, Sqlite};
use std::convert::Infallible;
use uuid::Uuid;

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
pub async fn delete_user(
    State(state): State<Pool<Sqlite>>,
    Json(req): Json<DeleteUserReq>,
) -> Result<Response, Infallible> {
    match delete_user_with_error(state, req).await {
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
async fn delete_user_with_error(
    pool: Pool<Sqlite>,
    req: DeleteUserReq,
) -> Result<String, anyhow::Error> {
    let query = format!(
        r#"SELECT * FROM user WHERE user_id = "{}""#,
        req.admin_user_id
    );
    let res = sqlx::query_as::<Sqlite, User>(&query)
        .fetch_optional(&pool)
        .await
        .map_err(|e| anyhow!("query error,the error is {}", e))?;

    match res {
        Some(user) => {
            sqlx::query("DELETE FROM user WHERE user_id = $1")
                .bind(req.user_id)
                .execute(&pool)
                .await
                .map_err(|e| anyhow!("delete error, the error is {}", e))?;
            to_string(0, String::from("User deleted successfully"))
        }
        None => to_string(-1, String::from("权限不允许!")),
    }
}
pub async fn add_user(
    State(state): State<Pool<Sqlite>>,
    Json(req): Json<AddUserReq>,
) -> Result<Response, Infallible> {
    match add_user_with_error(state, req).await {
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
async fn add_user_with_error(pool: Pool<Sqlite>, req: AddUserReq) -> Result<String, anyhow::Error> {
    let query = format!(
        r#"SELECT * FROM user WHERE user_id = "{}""#,
        req.admin_user_id
    );
    let res = sqlx::query_as::<Sqlite, User>(&query)
        .fetch_optional(&pool)
        .await
        .map_err(|e| anyhow!("query error,the error is {}", e))?;

    match res {
        Some(user) => {
            let uuid = Uuid::new_v4().to_string();

            sqlx::query("INSERT INTO user (user_account, user_password, user_authority, user_id) VALUES ($1, $2, $3, $4)")
                .bind(&req.user_account)
                .bind(&req.user_password)
                .bind(req.user_authority)
                .bind(&uuid)
                .execute(&pool)
                .await
                .map_err(|e| anyhow!("{}", e))?;

            to_string(0, String::from("User created successfully"))
        }
        None => to_string(-1, String::from("权限不允许!")),
    }
}
pub async fn update_user(
    State(state): State<Pool<Sqlite>>,
    Json(req): Json<UpdateUserReq>,
) -> Result<Response, Infallible> {
    match update_user_with_error(state, req).await {
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
async fn update_user_with_error(
    pool: Pool<Sqlite>,
    req: UpdateUserReq,
) -> Result<String, anyhow::Error> {
    let query = format!(
        r#"SELECT * FROM user WHERE user_id = "{}""#,
        req.admin_user_id
    );
    let res = sqlx::query_as::<Sqlite, User>(&query)
        .fetch_optional(&pool)
        .await
        .map_err(|e| anyhow!("query error,the error is {}", e))?;

    match res {
        Some(user) => {
            let uuid = Uuid::new_v4().to_string();
            let user_obj = User {
                user_account: req.user_account,
                user_password: req.user_password,
                user_authority: req.user_authority,
                ..user
            };

            let update_query = format!(
                r#"UPDATE user SET user_account = "{}", user_password = "{}", user_authority = {} WHERE user_id = "{}""#,
                user_obj.user_account,
                user_obj.user_password,
                user_obj.user_authority,
                user_obj.user_id
            );

            sqlx::query(&update_query)
                .execute(&pool)
                .await
                .map_err(|e| anyhow!("{}", e))?;

            to_string(0, String::from("User updated successfully"))
        }
        None => to_string(-1, String::from("权限不允许!")),
    }
}
