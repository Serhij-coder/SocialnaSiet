use axum::{Extension, Json};
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::json;

use crate::{
    db::{
        posts::{CreatePostArgs, create_post},
        topics::get_topic_id,
        users::{get_user_id, get_user_username},
    },
    image::{ImageType, save_image},
};

#[derive(Deserialize)]
pub struct CreatePostReq {
    pub topic: String,
    pub header: String,
    pub content: String,
    pub image: String,
}

pub async fn create_post_route(
    Extension(username): Extension<String>,
    req: Json<CreatePostReq>,
) -> (StatusCode, Json<serde_json::Value>) {
    let topic_id = match get_topic_id(&req.topic).await {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"Error": "Topic not found"})),
            );
        }
    };

    let user_id = match get_user_id(&username).await {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"Error": "User not found"})),
            );
        }
    };

    let image_file = if !req.image.is_empty() {
        match save_image(
            req.image.clone(),
            ImageType::Post {
                topic_name: req.topic.clone(),
            },
        )
        .await
        {
            Ok(name) => Some(name),
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"Error": e})),
                );
            }
        }
    } else {
        None
    };

    let content = if req.content.is_empty() {
        None
    } else {
        Some(req.content.clone())
    };

    let post_args = CreatePostArgs {
        topic_id,
        user_id,
        header: req.header.clone(),
        content,
        image: image_file,
    };

    match create_post(post_args).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"Ok": "Post created successfully"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"Error": e.to_string()})),
        ),
    }
}

#[derive(Deserialize)]
pub struct GetPostsReq {
    pub topic: String,
    pub amount: u32,
}

pub async fn get_posts_route(req: Json<GetPostsReq>) -> (StatusCode, Json<serde_json::Value>) {
    let topic_id = match get_topic_id(&req.topic).await {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"Error": "Topic not found"})),
            );
        }
    };

    let posts = match crate::db::posts::get_posts(topic_id, req.amount).await {
        Ok(posts) => posts,
        Err(e) => {
            println!("Error getting posts: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"Error": "Failed to get posts from db"})),
            );
        }
    };

    let mut processed_posts = Vec::new();
    for post in posts {
        let username = match get_user_username(post.user_id).await {
            Ok(u) => u,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"Error": "Failed to get username from db"})),
                );
            }
        };

        processed_posts.insert(
            0,
            json!({
                "username": username,
                "header": post.header,
                "content": post.content,
                "image": post.image,
                "timestamp": post.timestamp,
            }),
        );
    }

    (StatusCode::OK, Json(json!(processed_posts)))
}