use crate::models::post::{Post, PostWithUser};
use serde::Serialize;

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

impl From<Post> for PostResponse {
    fn from(p: Post) -> Self {
        Self {
            id: p.id,
            user_id: p.user_id,
            title: p.title,
            content: p.content,
            created_at: p.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct PostWithUserResponse {
    pub id: i64,
    pub user_id: i64,
    pub user_name: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

impl From<PostWithUser> for PostWithUserResponse {
    fn from(p: PostWithUser) -> Self {
        Self {
            id: p.id,
            user_id: p.user_id,
            user_name: p.user_name,
            title: p.title,
            content: p.content,
            created_at: p.created_at,
        }
    }
}
