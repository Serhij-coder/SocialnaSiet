use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::entities::{prelude::*, *};

use super::get_db;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePostArgs {
    pub topic_id: i32,
    pub user_id: i32,
    pub header: String,
    pub content: Option<String>,
    pub image: Option<String>,
}

pub async fn create_post(post: CreatePostArgs) -> Result<(), DbErr> {
    let new_post = posts::ActiveModel {
        topic_id: ActiveValue::Set(post.topic_id),
        user_id: ActiveValue::Set(post.user_id),
        header: ActiveValue::Set(post.header),
        content: ActiveValue::Set(post.content),
        image: ActiveValue::Set(post.image),
        ..Default::default()
    };

    Posts::insert(new_post)
        .exec(get_db())
        .await?;

    Ok(())
}

pub async fn get_posts(topic_id: i32, amount: u32) -> Result<Vec<posts::Model>, DbErr> {
    if amount == 0 {
        Posts::find()
            .filter(posts::Column::TopicId.eq(topic_id))
            .order_by_desc(posts::Column::Timestamp)
            .all(get_db())
            .await
    } else {
        Posts::find()
            .filter(posts::Column::TopicId.eq(topic_id))
            .order_by_desc(posts::Column::Timestamp)
            .limit(amount as u64)
            .all(get_db())
            .await
    }
}