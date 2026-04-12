use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::entities::{prelude::*, *};

use super::get_db;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub role: String,
    pub profile_picture: String,
}

pub async fn create_user(user: User) -> Result<(), DbErr> {
    let profile_picture = if user.profile_picture.is_empty() {
        ActiveValue::NotSet
    } else {
        ActiveValue::Set(user.profile_picture.to_owned().into())
    };

    let new_user = users::ActiveModel {
        username: ActiveValue::Set(user.username.to_owned()),
        password: ActiveValue::Set(user.password.to_owned()),
        nickname: ActiveValue::Set(user.nickname.to_owned()),
        role: ActiveValue::Set(user.role.to_owned()),
        profile_picture,
        ..Default::default()
    };

    let _res = Users::insert(new_user).exec(get_db()).await?;

    Ok(())
}

pub async fn is_user(username: &str) -> Result<bool, DbErr> {
    let user = Users::find()
        .filter(users::Column::Username.eq(username))
        .count(get_db())
        .await;

    match user {
        Err(err) => Err(err),
        Ok(value) if value != 0 => Ok(true),
        _ => Ok(false),
    }
}
