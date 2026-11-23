use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiError {
    pub code: &'static str,
    pub message: &'static str,
}

pub type ApiResult<T> = Result<T, (StatusCode, Json<ApiError>)>;

pub fn error(
    status: StatusCode,
    code: &'static str,
    message: &'static str,
) -> (StatusCode, Json<ApiError>) {
    (status, Json(ApiError { code, message }))
}
