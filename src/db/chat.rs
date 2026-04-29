use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::entities::{prelude::*, *};

use super::get_db;

pub struct CreateMessageArgs {
    pub topic_id: i32,
    pub user_id: i32,
    pub message: Option<String>,
    pub image: Option<String>,
}

/// Creates a new message entry in the database.
pub async fn create_message(message: CreateMessageArgs) -> Result<(), ()> {
    let new_message = chat::ActiveModel {
        topic_id: ActiveValue::Set(message.topic_id),
        user_id: ActiveValue::Set(message.user_id),
        message: ActiveValue::Set(message.message),
        image: ActiveValue::Set(message.image),
        ..Default::default()
    };

    Chat::insert(new_message)
        .exec(get_db())
        .await
        .map_err(|_| ())?;

    Ok(())
}

/// Retrieves messages for a given topic, sorted by timestamp in descending order, and limited by
/// the specified amount. 0 to get all messages.
pub async fn get_messages(topic_id: i32, amount: u32) -> Result<Vec<chat::Model>, DbErr> {
    if amount == 0 {
        Chat::find()
            .filter(chat::Column::TopicId.eq(topic_id))
            .order_by_desc(chat::Column::Timestamp)
            .all(get_db())
            .await
    } else {
        Chat::find()
            .filter(chat::Column::TopicId.eq(topic_id))
            .order_by_desc(chat::Column::Timestamp)
            .limit(amount as u64)
            .all(get_db())
            .await
    }
}
