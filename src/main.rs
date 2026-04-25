use axum::{
    Router,
    http::{
        Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    middleware::from_fn,
    routing::{get, post},
};
use dashmap::DashMap;
use tokio::net::TcpListener;

use dotenvy::dotenv;
use std::{env, panic, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

mod api;
mod db;
mod entities;
mod image;

use api::auth::{check_jwt, login};
use api::topics::{get_topics, new_topic};
use api::users::{check_username_availability, init_admin_user, new_public_user};

use crate::api::chat::{ChatState, append_message_route, get_ignored_message, sse_handler};

fn check_env_vars() {
    env::var("DATABASE_URL")
        .expect("Failed to load DATABASE_URL. Ensure variable DATABASE_URL exist in .env");

    let data_dir = env::var("DATA_DIR")
        .expect("Failed to load DATA_DIR. Ensure variable DATA_DIR exist in .env");
    if data_dir == "/" {
        panic!("DATA_DIR can't be \"/\"");
    }

    env::var("JWT_SECRET")
        .expect("Failed to load JWT_SECRET. Ensure variable JWT_SECRET exist in .env");

    env::var("ADMIN_PASSWORD")
        .expect("Failed to load ADMIN_PASSWORD. Ensure variable ADMIN_PASSWORD exist in .env");

    env::var("CONNECTION_TEST_CHAT_STRING")
        .expect("Failed to load CONNECTION_TEST_CHAT_STRING. Ensure variable CONNECTION_TEST_CHAT_STRING exist in .env");
}

#[tokio::main]
async fn main() {
    println!("========Socialna Siet========");

    dotenv().expect("Error loading .env file. Ensure .env file exist");
    check_env_vars();

    let data_dir = env::var("DATA_DIR").unwrap();

    if let Err(e) = tokio::fs::create_dir_all(&data_dir).await {
        panic!("Failed to create DATA_DIR at {}: {}", data_dir, e);
    }

    let shared_state = Arc::new(ChatState {
        stream: DashMap::new(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    tracing_subscriber::fmt::init();

    db::init_db().await;

    match init_admin_user().await {
        Ok(message) => println!("{}", message),
        Err(_) => panic!("Failed to init admin user"),
    }

    let app = Router::new()
        .route("/create_topic", post(new_topic))
        .route("/append_message", post(append_message_route))
        .layer(from_fn(check_jwt))
        .route("/create_user", post(new_public_user))
        .route(
            "/check_username_availability",
            post(check_username_availability),
        )
        .route("/get_topics", get(get_topics))
        .route("/get_ignored_message", get(get_ignored_message))
        .route("/login", post(login))
        .route("/chat", get(sse_handler))
        .with_state(shared_state)
        .nest_service("/res", ServeDir::new(data_dir))
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Listen on {} try connect to 127.0.0.1:3000",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
