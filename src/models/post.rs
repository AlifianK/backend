use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Deserialize, Validate)]
pub struct CreatePost {
    pub user_id: i64,
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
}

impl Post {
    pub async fn find_all(pool: &SqlitePool) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM posts")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Self> {
        sqlx::query_as::<_, Self>("SELECT * FROM posts WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn find_by_user(pool: &SqlitePool, user_id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM posts WHERE user_id = ?")
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(pool: &SqlitePool, data: CreatePost) -> sqlx::Result<Self> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO posts (user_id, title, content) VALUES (?, ?, ?) RETURNING *",
        )
        .bind(data.user_id)
        .bind(data.title)
        .bind(data.content)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM posts WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await
            .map(|_| ())
    }
}
