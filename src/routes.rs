use crate::controllers::post_controller;
use crate::controllers::user_controller;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/users/{id}",
            get(user_controller::get_user)
                .delete(user_controller::delete_user)
                .patch(user_controller::update_user),
        )
        .route(
            "/users",
            get(user_controller::get_all_users).post(user_controller::create_user),
        )
        .route(
            "/posts",
            get(post_controller::get_all_posts).post(post_controller::create_post),
        )
        .route(
            "/posts/{id}",
            get(post_controller::get_post).delete(post_controller::delete_post),
        )
        .route(
            "/users/{user_id}/posts",
            get(post_controller::get_posts_by_user),
        )
        .fallback_service(ServeDir::new("static"))
        .with_state(pool)
}
