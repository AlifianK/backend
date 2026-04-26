use crate::models::post::Post;
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
