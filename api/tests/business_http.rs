use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};

use platform_api::{app::create_router, state::AppState};
use serde_json::Value;
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

#[sqlx::test(migrations = "./migrations")]
async fn creates_and_reads_business_over_http(pool: PgPool) {
    let router = create_router(AppState {
        pool,
        local_only: true,
    });

    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/dev/seed-business")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let created: Value = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(created["name"], "Demo Candle Shop");

    let id = Uuid::parse_str(created["id"].as_str().unwrap()).unwrap();

    let response = router
        .oneshot(
            Request::builder()
                .uri(format!("/v1/dev/businesses/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    let retrieved: Value = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(retrieved["name"], created["name"]);
    assert_eq!(retrieved["id"], created["id"]);
}

#[sqlx::test(migrations = "./migrations")]
async fn returns_404_for_missing_business(pool: PgPool) {
    let router = create_router(AppState {
        pool,
        local_only: true,
    });

    let missing_id = Uuid::new_v4();
    let reponse = router
        .oneshot(
            Request::builder()
                .uri(format!("/v1/dev/businesses/{missing_id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(reponse.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(migrations = "./migrations")]
async fn blocks_dev_endpooints_outside_local_mode(pool: PgPool) {
    let router = create_router(AppState {
        pool,
        local_only: false,
    });

    let response = router
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/dev/seed-business")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let response = router
        .oneshot(
            Request::builder()
                .uri(format!("/v1/dev/businesses/{}", Uuid::new_v4()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
