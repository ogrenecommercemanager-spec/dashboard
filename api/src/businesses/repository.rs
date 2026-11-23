
use sqlx::PgPool;
use uuid::Uuid;

use super::models::Business;

pub async fn create_business(
    pool: &PgPool,
    name: &str,
) -> Result<Business, sqlx::Error> {
    let id = Uuid::new_v4();

    sqlx::query_as::<_, Business>(
        "INSERT INTO businesses (id, name)
         VALUES ($1, $2)
         RETURNING id, name",
    )
    .bind(id)
    .bind(name)
    .fetch_one(pool)
    .await
}

pub async fn find_business(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Business>, sqlx::Error> {
    sqlx::query_as::<_, Business>(
        "SELECT id, name
         FROM businesses
         WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}
