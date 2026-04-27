use crate::models::comment::Comment;
use serde::Serialize;

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: i64,
    pub post_id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: String,
}

impl From<Comment> for CommentResponse {
    fn from(c: Comment) -> Self {
        Self {
            id: c.id,
            post_id: c.post_id,
            user_id: c.user_id,
            content: c.content,
            created_at: c.created_at,
        }
    }
}
