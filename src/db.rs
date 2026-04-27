use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn connect() -> SqlitePool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .after_connect(|conn, _| {
            Box::pin(async move {
                sqlx::query("PRAGMA foreign_keys = ON")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&url)
        .await
        .expect("Failed to connect")
}
