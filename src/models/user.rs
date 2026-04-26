use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}

impl User {
    pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM users")
            .fetch_all(pool)
            .await
    }

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

    pub async fn update(pool: &SqlitePool, id: i64, data: UpdateUser) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            "UPDATE users SET
                name  = COALESCE(?, name),
                email = COALESCE(?, email)
            WHERE id = ? RETURNING *",
            data.name,
            data.email,
            id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> sqlx::Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(pool)
            .await
            .map(|_| ())
    }
}
