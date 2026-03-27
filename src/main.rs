use axum::{
    Error, Json, Router,
    http::{Method, Result, header::CONTENT_TYPE},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use dotenvy::dotenv;
use std::env;
use tower_http::cors::{Any, CorsLayer};

mod api;
mod db;
mod entities;

use api::users::{check_username_availability, new_user};

#[derive(Deserialize, Serialize, Debug)]
struct Test {
    message: String,
    value: i32,
}

async fn check_env_vars() {
    env::var("DATABASE_URL")
        .expect("Failed to load DATABASE_URL. Ensure variable DATABASE_URL exist in .env");
}

#[tokio::main]
async fn main() {
    dotenv().expect("Error loading .env file. Ensure .env file exist");
    check_env_vars();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    tracing_subscriber::fmt::init();

    db::init_db().await;

    let app = Router::new()
        .route("/create_user", post(new_user))
        .route(
            "/check_username_availability",
            post(check_username_availability),
        )
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Listen on {} try connect to 127.0.0.1:3000",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
