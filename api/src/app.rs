use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    businesses::handlers,
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route(
            "/v1/dev/seed-business",
            post(handlers::seed_business),
        )
        .route(
            "/v1/dev/businesses/{id}",
            get(handlers::get_business),
        )
        .with_state(state)
}