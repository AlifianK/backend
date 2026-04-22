use crate::models::user::{CreateUser, UpdateUser, User};
use crate::views::user_view::UserResponse;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::SqlitePool;

pub async fn get_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<UserResponse>, StatusCode> {
    User::find_by_id(&pool, id)
        .await
        .map(|u| Json(UserResponse::from(u)))
        .map_err(|_| StatusCode::NOT_FOUND)
}

pub async fn create_user(
    State(pool): State<SqlitePool>,
    Json(body): Json<CreateUser>,
) -> Result<Json<UserResponse>, StatusCode> {
    User::create(&pool, body)
        .await
        .map(|u| Json(UserResponse::from(u)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn delete_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    User::delete(&pool, id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_user(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateUser>,
) -> Result<Json<UserResponse>, StatusCode> {
    User::update(&pool, id, body)
        .await
        .map(|u| Json(UserResponse::from(u)))
        .map_err(|_| StatusCode::NOT_FOUND)
}
