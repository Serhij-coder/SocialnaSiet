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
