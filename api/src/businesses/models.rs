
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Business {
    pub id: Uuid,
    pub name: String,
}
