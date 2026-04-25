use axum::{
    Extension, Json,
    extract::{Query, State},
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
};
use dashmap::DashMap;
use futures_util::Stream;
use serde::Deserialize;
use serde_json::json;
use std::{
    convert::Infallible,
    env,
    sync::{Arc, mpsc::Sender},
    time::Duration,
};
use tokio::{sync::broadcast, time::interval};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tracing::event;

use crate::{
    db::{
        chat::{CreateMessageArgs, create_message},
        topics::get_topic_id,
        users::get_user_id,
    },
    image::{self, ImageType, save_image},
};

#[derive(Deserialize)]
pub struct TopicPagination {
    pub topic: String,
}

pub struct ChatState {
    pub stream: DashMap<String, broadcast::Sender<Event>>,
}

#[derive(Deserialize)]
pub struct MessegeReq {
    topic: String,
    message: String,
    image: String,
}

pub enum MessageType {
    TextOnly(String),
    TextImage(String, String),
    ImageOnly(String),
}

pub async fn get_ignored_message() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(json!({"message": env::var("CONNECTION_TEST_CHAT_STRING").unwrap()})),
    )
}

pub async fn sse_handler(
    Query(params): Query<TopicPagination>,
    State(state): State<Arc<ChatState>>,
) -> Sse<impl Stream<Item = std::result::Result<Event, Infallible>>> {
    let topic_name = params.topic;

    let tx = state
        .stream
        .entry(topic_name.clone())
        .or_insert_with(|| {
            let (tx, _) = broadcast::channel(16);
            let tx_internal = tx.clone();
            let state_for_task = state.clone();

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(10));
                loop {
                    interval.tick().await;
                    let msg = format!("{}", env::var("CONNECTION_TEST_CHAT_STRING").unwrap(),);

                    if tx_internal.send(Event::default().data(msg)).is_err() {
                        dbg!("No subscriber for {}, cleaning up", &topic_name);
                        state_for_task.stream.remove(&topic_name);
                        break;
                    }
                }
            });
            tx
        })
        .clone();

    let rx = tx.subscribe();
    let stream = BroadcastStream::new(rx).map(|result| match result {
        Ok(event) => Ok(event),
        Err(_) => Ok(Event::default().data("lagged")),
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// Check for data correctness and append_message if all right
pub async fn append_message_route(
    Extension(username): Extension<String>,
    State(state): State<Arc<ChatState>>,
    req: Json<MessegeReq>,
) -> (StatusCode, Json<serde_json::Value>) {
    let half_err_message = |err_msg: &str| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(
                json!({"Ok": "Succesfuly saved message", "Error": "Error sending message to stream", "Error Message": err_msg}),
            ),
        )
    };
    match match (req.message.as_str(), req.image.as_str()) {
        ("", "") => Err(()),
        (msg, "") => Ok(MessageType::TextOnly(msg.to_string())),
        ("", img) => Ok(MessageType::ImageOnly(img.to_string())),
        (msg, img) => Ok(MessageType::TextImage(msg.to_string(), img.to_string())),
    } {
        Err(_) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"Error": "Provide message or image"})),
        ),
        Ok(message_type) => {
            match append_message(message_type, req.topic.as_ref(), &username).await {
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"Error": e}))),
                Ok(stream_message) => match state.stream.get(req.topic.as_str()) {
                    Some(tx) => match tx.send(Event::default().data(stream_message.to_string())) {
                        Ok(_) => (
                            StatusCode::OK,
                            Json(
                                json!({"Ok": "Message saved succesfully", "Okk": "Message sended succesfully"}),
                            ),
                        ),
                        Err(e) => half_err_message(e.to_string().as_str()),
                    },
                    None => half_err_message("Topic not found in streams hashmap"),
                },
            }
        }
    }
}

/// Save image and write message to db depend on MessageType
async fn append_message(
    message: MessageType,
    topic: &str,
    username: &str,
) -> Result<Json<serde_json::Value>, String> {
    let topic_id = get_topic_id(topic)
        .await
        .map_err(|_| "Failed get topic id".to_string())?;
    let user_id = get_user_id(username)
        .await
        .map_err(|_| "Failed get user id".to_string())?;
    dbg!("begin append message");

    // Save image if present
    let image_file = if let MessageType::TextImage(_, image_b64)
    | MessageType::ImageOnly(image_b64) = &message
    {
        dbg!("imge found");
        Some(
            save_image(
                image_b64.to_string(),
                ImageType::Chat {
                    topic_name: topic.to_string(),
                },
            )
            .await
            .map_err(|_| "Error saving image".to_string())?,
        )
    } else {
        dbg!("imge not found");
        None
    };

    let message_args = match message {
        MessageType::TextOnly(text) => CreateMessageArgs {
            topic_id,
            user_id,
            message: Some(text),
            image: image_file,
        },
        MessageType::TextImage(text, _) => CreateMessageArgs {
            topic_id,
            user_id,
            message: Some(text),
            image: image_file,
        },
        MessageType::ImageOnly(_) => CreateMessageArgs {
            topic_id,
            user_id,
            message: None,
            image: image_file,
        },
    };

    let stream_message = Json(json!({
        "message": message_args.message,
        "image": message_args.image,
    }));

    create_message(message_args)
        .await
        .map_err(|_| "Failed write message to db".to_string())?;

    Ok(stream_message)
}
