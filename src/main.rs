use std::println;

use axum::{
    Error, Json, Router,
    http::Result,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

mod api;
mod db;
mod entities;

use api::users::new_user;

#[derive(Deserialize, Serialize, Debug)]
struct Test {
    message: String,
    value: i32,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    db::init_db().await;

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/create_user", post(new_user))
        .route("/test", get(compose_response))
        .route("/increment", post(increment));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Listen on {} try connect to 127.0.0.1:3000",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn increment(Json(request): Json<Test>) -> Json<Test> {
    let test = request;
    let mut value = test.value;
    value += 1;

    dbg!(test);

    let response = Test {
        message: String::from("Your incremented value."),
        value,
    };

    Json(response)
}

async fn compose_response() -> Json<Test> {
    let test = Test {
        message: String::from("This is a test message"),
        value: 32,
    };

    Json(test)
}

async fn root_handler() -> &'static str {
    "Hello, Axum!"
}
