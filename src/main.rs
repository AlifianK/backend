#![allow(unused)]
mod db;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = db::connect().await;
}
