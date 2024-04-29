use crate::vojo::base_response::BaseResponse;
use axum::extract::State;
use sqlx::{Pool, Sqlite};
use std::convert::Infallible;
pub async fn get_route(
    State(state): State<Pool<Sqlite>>,
) -> Result<impl axum::response::IntoResponse, Infallible> {
    match get_route_with_error(state).await {
        Ok(r) => Ok((axum::http::StatusCode::OK, r)),
        Err(e) => Ok((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
async fn get_route_with_error(pool: Pool<Sqlite>) -> Result<String, anyhow::Error> {
    let data = BaseResponse {
        response_code: 0,
        response_object: 0,
    };
    Ok(serde_json::to_string(&data).unwrap())
}
