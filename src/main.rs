#![allow(unused)]
mod db;
mod models;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = db::connect().await;
}
