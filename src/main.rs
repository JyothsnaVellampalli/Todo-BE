use axum::{self};
use dotenvy::dotenv;

mod app;
mod models;
mod persistence;
mod handlers;
mod auth;
mod logging;

use app::prepare_app;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    dotenv().ok();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    let app = prepare_app().await;

    axum::serve(listener, app).await.unwrap();
}