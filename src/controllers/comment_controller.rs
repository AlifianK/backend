use crate::errors::AppError;
use crate::models::comment::{Comment, CreateComment, UpdateComment};
use crate::views::comment_view::CommentResponse;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::SqlitePool;
use validator::Validate;

pub async fn get_comments_by_post(
    State(pool): State<SqlitePool>,
    Path(post_id): Path<i64>,
) -> Result<Json<Vec<CommentResponse>>, AppError> {
    Comment::find_by_post(&pool, post_id)
        .await
        .map(|comments| Json(comments.into_iter().map(CommentResponse::from).collect()))
        .map_err(AppError::from)
}

pub async fn create_comment(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateComment>,
) -> Result<Json<CommentResponse>, AppError> {
    body.validate().map_err(AppError::from)?;

    Comment::create(&pool, body)
        .await
        .map(|c| Json(CommentResponse::from(c)))
        .map_err(AppError::from)
}

pub async fn update_comment(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateComment>,
) -> Result<Json<CommentResponse>, AppError> {
    body.validate().map_err(AppError::from)?;

    Comment::update(&pool, id, body)
        .await
        .map(|c| Json(CommentResponse::from(c)))
        .map_err(AppError::from)
}

pub async fn delete_comment(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    Comment::delete(&pool, id).await.map_err(AppError::from)
}
