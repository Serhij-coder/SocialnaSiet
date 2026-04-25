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
    let topics = Topics::find().into_model::<Topic>().all(get_db()).await?;
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
