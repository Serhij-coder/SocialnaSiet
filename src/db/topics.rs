use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::entities::{prelude::*, *};

use super::get_db;

#[derive(Deserialize, Serialize)]
pub struct Topic {
    pub name: String,
    pub no_spaces_name: String,
}

pub async fn create_topic(topic: Topic) -> Result<(), DbErr> {
    let new_topic = topics::ActiveModel {
        name: ActiveValue::Set(topic.name.to_owned()),
        no_spaces_name: ActiveValue::Set(topic.no_spaces_name.to_owned()),
        ..Default::default()
    };

    let _res = Topics::insert(new_topic).exec(get_db()).await?;

    Ok(())
}
