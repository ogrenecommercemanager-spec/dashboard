use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    error::{error, ApiResult},
    state::AppState,
};

use super::{models::Business, service};

pub async fn seed_business(
    State(state): State<AppState>,
) -> ApiResult<(StatusCode, Json<Business>)> {
    if !state.local_only {
        return Err(error(StatusCode::NOT_FOUND, "NOT_FOUND", "Not found"));
    }

    let business = service::create_demo_business(&state.pool)
        .await
        .map_err(|_| {
            error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "DB_ERROR",
                "Database request failed",
            )
        })?;

    Ok((StatusCode::CREATED, Json(business)))
}

pub async fn get_business(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Business>> {
    if !state.local_only {
        return Err(error(StatusCode::NOT_FOUND, "NOT_FOUND", "Not found"));
    }

    let found = service::get_business(&state.pool, id).await.map_err(|_| {
        error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "DB_ERROR",
            "Database request failed",
        )
    })?;

    found
        .map(Json)
        .ok_or_else(|| error(StatusCode::NOT_FOUND, "NOT_FOUND", "Business not found"))
}
