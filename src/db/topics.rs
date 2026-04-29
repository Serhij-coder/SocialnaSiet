use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::entities::{prelude::*, *};

use super::get_db;

#[derive(Deserialize, Serialize, FromQueryResult)]
pub struct Topic {
    pub name: String,
    pub no_spaces_name: String,
    pub topic_picture: String,
}

pub async fn create_topic(topic: Topic) -> Result<(), DbErr> {
    let topic_picture = if topic.topic_picture.is_empty() {
        ActiveValue::NotSet
    } else {
        ActiveValue::Set(topic.topic_picture.to_owned().into())
    };

    let new_topic = topics::ActiveModel {
        name: ActiveValue::Set(topic.name.to_owned()),
        no_spaces_name: ActiveValue::Set(topic.no_spaces_name.to_owned()),
        topic_picture,
        ..Default::default()
    };

    let _res = Topics::insert(new_topic).exec(get_db()).await?;

    Ok(())
}

pub async fn get_all_topics() -> Result<Vec<Topic>, DbErr> {
    dbg!("Try to get topics");
    // 1. Query as a Raw Json Value to allow NULLs
    let raw_topics = Topics::find().into_json().all(get_db()).await?;

    // 2. Manually map the JSON into your Topic struct
    let topics = raw_topics
        .into_iter()
        .map(|json| {
            Topic {
                name: json["name"].as_str().unwrap_or("").to_string(),
                no_spaces_name: json["no_spaces_name"].as_str().unwrap_or("").to_string(),
                // If it's null, we force it to ""
                topic_picture: json["topic_picture"].as_str().unwrap_or("").to_string(),
            }
        })
        .collect();
    dbg!("Topics SELECTED succesfully");

    Ok(topics)
}

pub async fn get_topic_id(topic: &str) -> Result<i32, ()> {
    match Topics::find()
        .filter(topics::Column::Name.eq(topic))
        .one(get_db())
        .await
        .map_err(|_| ())?
    {
        Some(s) => Ok(s.id),
        None => Err(()),
    }
}
