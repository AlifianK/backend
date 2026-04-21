use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn connect() -> SqlitePool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Failed to connect")
}
