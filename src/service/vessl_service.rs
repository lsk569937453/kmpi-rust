use crate::vojo::base_response::BaseResponse;
use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;
use axum::response::Response;
use sqlx::{Pool, Sqlite};
use std::convert::Infallible;
pub async fn get_route(State(state): State<Pool<Sqlite>>) -> Result<Response, Infallible> {
    match get_route_with_error(state).await {
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
async fn get_route_with_error(pool: Pool<Sqlite>) -> Result<String, anyhow::Error> {
    let data = BaseResponse {
        response_code: 0,
        response_object: 0,
    };
    serde_json::to_string(&data).map_err(|e| anyhow!("{}", e))
}
