use axum::{
    Router,
    extract::Query,
    http::{Method, header::CONTENT_TYPE},
    response::sse::{Event, Sse},
    routing::{get, post},
};
use futures_util::Stream;
use tokio::net::TcpListener;
use tokio_stream::StreamExt as _;

use dotenvy::dotenv;
use std::{convert::Infallible, env, time::Duration};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

mod api;
mod db;
mod entities;
mod image;

use api::topics::{get_topics, new_topic};
use api::users::{check_username_availability, new_user};

use serde::Deserialize;

#[derive(Deserialize)]
struct TopicPagination {
    topic: String,
}

fn check_env_vars() {
    env::var("DATABASE_URL")
        .expect("Failed to load DATABASE_URL. Ensure variable DATABASE_URL exist in .env");

    env::var("DATA_DIR").expect("Failed to load DATA_DIR. Ensure variable DATA_DIR exist in .env");
}

#[tokio::main]
async fn main() {
    dotenv().expect("Error loading .env file. Ensure .env file exist");
    check_env_vars();

    let data_dir = env::var("DATA_DIR").unwrap();

    if let Err(e) = tokio::fs::create_dir_all(&data_dir).await {
        panic!("Failed to create DATA_DIR at {}: {}", data_dir, e);
    }

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
        .route("/create_topic", post(new_topic))
        .route("/get_topics", get(get_topics))
        .route("/stream", get(sse_handler))
        .nest_service("/res", ServeDir::new(data_dir))
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Listen on {} try connect to 127.0.0.1:3000",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn sse_handler(
    Query(params): Query<TopicPagination>,
) -> Sse<impl Stream<Item = std::result::Result<Event, Infallible>>> {
    let topic = params.topic;
    println!("Client requested topic: {}", topic);

    let stream =
        tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
            .map(move |_| {
                let data = format!("New update for {}", topic);
                Ok(Event::default().data(data))
            });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new())
}
