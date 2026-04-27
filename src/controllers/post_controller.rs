use crate::errors::AppError;
use crate::models::post::{CreatePost, Post, UpdatePost};
use crate::views::post_view::{PostResponse, PostWithUserResponse};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::SqlitePool;
use validator::Validate;

pub async fn get_all_posts(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<PostWithUserResponse>>, AppError> {
    Post::find_all_with_user(&pool)
        .await
        .map(|posts| Json(posts.into_iter().map(PostWithUserResponse::from).collect()))
        .map_err(AppError::from)
}

pub async fn get_post(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<PostResponse>, AppError> {
    Post::find_by_id(&pool, id)
        .await
        .map(|p| Json(PostResponse::from(p)))
        .map_err(AppError::from)
}

pub async fn get_posts_by_user(
    State(pool): State<SqlitePool>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<PostResponse>>, AppError> {
    Post::find_by_user(&pool, user_id)
        .await
        .map(|posts| Json(posts.into_iter().map(PostResponse::from).collect()))
        .map_err(AppError::from)
}

pub async fn create_post(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreatePost>,
) -> Result<Json<PostResponse>, AppError> {
    body.validate().map_err(AppError::from)?;

    Post::create(&pool, body)
        .await
        .map(|p| Json(PostResponse::from(p)))
        .map_err(AppError::from)
}

pub async fn update_post(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(body): Json<UpdatePost>,
) -> Result<Json<PostResponse>, AppError> {
    body.validate().map_err(AppError::from)?;

    Post::update(&pool, id, body)
        .await
        .map(|p| Json(PostResponse::from(p)))
        .map_err(AppError::from)
}

pub async fn delete_post(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    Post::delete(&pool, id).await.map_err(AppError::from)
}
