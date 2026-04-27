use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Comment {
    pub id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: String,
}

#[derive(Deserialize, Validate)]
pub struct CreateComment {
    pub post_id: i64,
    pub user_id: i64,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
}

impl Comment {
    pub async fn find_by_post(pool: &SqlitePool, post_id: i64) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>("SELECT * FROM comments WHERE post_id = ?")
            .bind(post_id)
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(pool: &SqlitePool, id: i64) -> sqlx::Result<Self> {
        sqlx::query_as::<_, Self>("SELECT * FROM comments WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn create(pool: &SqlitePool, data: CreateComment) -> sqlx::Result<Self> {
        sqlx::query_as::<_, Self>(
            "INSERT INTO comments (post_id, user_id, content) VALUES (?, ?, ?) RETURNING *",
        )
        .bind(data.post_id)
        .bind(data.user_id)
        .bind(data.content)
        .fetch_one(pool)
        .await
    }
}
