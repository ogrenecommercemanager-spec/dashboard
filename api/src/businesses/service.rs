use sqlx::PgPool;
use uuid::Uuid;

use super::{models::Business, repository};

pub async fn create_demo_business(pool: &PgPool) -> Result<Business, sqlx::Error> {
    repository::create_business(pool, "Demo Candle Shop").await
}

pub async fn get_business(pool: &PgPool, id: Uuid) -> Result<Option<Business>, sqlx::Error> {
    repository::find_business(pool, id).await
}
