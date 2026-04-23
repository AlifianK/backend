use crate::errors::AppError;
use crate::models::user::{CreateUser, UpdateUser, User};
use crate::views::user_view::UserResponse;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::SqlitePool;
use validator::Validate;

pub async fn get_all_users(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    User::find_all(&pool)
        .await
        .map(|users| Json(users.into_iter().map(UserResponse::from).collect()))
        .map_err(AppError::from)
}

pub async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<UserResponse>, AppError> {
    User::find_by_id(&pool, id)
        .await
        .map(|u| Json(UserResponse::from(u)))
        .map_err(AppError::from)
}

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateUser>,
) -> Result<Json<UserResponse>, AppError> {
    body.validate().map_err(AppError::from)?;

    User::create(&pool, body)
        .await
        .map(|u| Json(UserResponse::from(u)))
        .map_err(AppError::from)
}

pub async fn delete_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    User::delete(&pool, id).await.map_err(AppError::from)
}

pub async fn update_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateUser>,
) -> Result<Json<UserResponse>, AppError> {
    body.validate().map_err(AppError::from)?;

    User::update(&pool, id, body)
        .await
        .map(|u| Json(UserResponse::from(u)))
        .map_err(AppError::from)
}
