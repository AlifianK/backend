use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

impl User {
    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Self> {
        sqlx::query_as!(Self, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(pool)
            .await
    }

    pub async fn create(pool: &SqlitePool, data: CreateUser) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "INSERT INTO users (name, email) VALUES (?, ?) RETURNING *",
            data.name,
            data.email
        )
        .fetch_one(pool)
        .await
    }
}
