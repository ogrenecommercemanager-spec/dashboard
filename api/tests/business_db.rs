use sqlx::PgPool;
use uuid::Uuid;

#[sqlx::test(migrations = "./migrations")]
async fn creates_and_reads_business(pool: PgPool) {
    let id = Uuid::new_v4();
    let name = "Integration Test Shop";

    sqlx::query("INSERT INTO businesses (id, name) VALUES ($1, $2)")
        .bind(id)
        .bind(name)
        .execute(&pool)
        .await
        .expect("Failed to insert business");

    let business: (Uuid, String) = sqlx::query_as("SELECT id, name FROM businesses WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .expect("Failed to retrieve business");

    assert_eq!(business.0, id);
    assert_eq!(business.1, name);
}
