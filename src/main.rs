#![allow(unused)]
mod controllers;
mod db;
mod models;
mod routes;
mod views;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = db::connect().await;
}
