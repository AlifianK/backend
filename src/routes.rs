use crate::controllers::user_controller;
use axum::{
    Router,
    routing::{delete, get, post},
};
use sqlx::SqlitePool;

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/users/{id}",
            get(user_controller::get_user).delete(user_controller::delete_user),
        )
        .route("/users", post(user_controller::create_user))
        .with_state(pool)
}
