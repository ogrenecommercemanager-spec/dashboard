use axum::{extract::{Path, State}, http::StatusCode, routing::{get, post}, Json, Router};
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::{env, net::SocketAddr};
use uuid::Uuid;

#[derive(Clone)]
struct AppState { pool: PgPool, local_only: bool }

#[derive(Serialize, sqlx::FromRow)]
struct Business { id: Uuid, name: String }

#[derive(Serialize)]
struct ApiError { code: &'static str, message: &'static str }

type ApiResult<T> = Result<T, (StatusCode, Json<ApiError>)>;

fn error(status: StatusCode, code: &'static str, message: &'static str) -> (StatusCode, Json<ApiError>) {
    (status, Json(ApiError { code, message }))
}

async fn healthz() -> &'static str { "ok" }

async fn seed_business(State(state): State<AppState>) -> ApiResult<(StatusCode, Json<Business>)> {
    if !state.local_only {
        return Err(error(StatusCode::NOT_FOUND, "NOT_FOUND", "Not found"));
    }
    let id = Uuid::new_v4();
    let business = sqlx::query_as::<_, Business>(
        "INSERT INTO businesses (id, name) VALUES ($1, $2) RETURNING id, name"
    )
    .bind(id)
    .bind("Demo Candle Shop")
    .fetch_one(&state.pool).await
    .map_err(|_| error(StatusCode::INTERNAL_SERVER_ERROR, "DB_ERROR", "Database request failed"))?;
    Ok((StatusCode::CREATED, Json(business)))
}

async fn get_business(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<Business>> {
    if !state.local_only {
        return Err(error(StatusCode::NOT_FOUND, "NOT_FOUND", "Not found"));
    }
    let found = sqlx::query_as::<_, Business>("SELECT id, name FROM businesses WHERE id = $1")
        .bind(id).fetch_optional(&state.pool).await
        .map_err(|_| error(StatusCode::INTERNAL_SERVER_ERROR, "DB_ERROR", "Database request failed"))?;
    found.map(Json).ok_or_else(|| error(StatusCode::NOT_FOUND, "NOT_FOUND", "Business not found"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info".into())).init();
    let url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().max_connections(5).connect(&url).await?;
    sqlx::migrate!().run(&pool).await?;
    let state = AppState { pool, local_only: env::var("APP_ENV").as_deref() == Ok("local") };
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/v1/dev/seed-business", post(seed_business))
        .route("/v1/dev/businesses/{id}", get(get_business))
        .with_state(state);
    let address: SocketAddr = "0.0.0.0:8080".parse()?;
    tracing::info!(%address, "starting API");
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
