#![allow(unused)]
mod controllers;
mod db;
mod errors;
mod models;
mod routes;
mod views;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = db::connect().await;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migration failed");
    let app = routes::create_router(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
