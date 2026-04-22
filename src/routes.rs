use crate::controllers::user_controller;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use sqlx::SqlitePool;

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/users/{id}",
            get(user_controller::get_user)
                .delete(user_controller::delete_user)
                .patch(user_controller::update_user),
        )
        .route("/users", post(user_controller::create_user))
        .with_state(pool)
}
