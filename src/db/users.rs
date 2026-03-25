use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::entities::{prelude::*, *};

use super::get_db;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub nickname: String,
}

pub async fn create_user(user: User) -> Result<(), DbErr> {
    let new_user = users::ActiveModel {
        username: ActiveValue::Set(user.username.to_owned()),
        password: ActiveValue::Set(user.password.to_owned()),
        nickname: ActiveValue::Set(user.nickname.to_owned()),
        ..Default::default()
    };

    let res = Users::insert(new_user).exec(get_db()).await?;

    Ok(())
}
