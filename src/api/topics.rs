use axum::Json;
use axum::http::StatusCode;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::db::topics::Topic as DB_Topic;
use crate::db::topics::create_topic;

#[derive(Serialize, Deserialize)]
pub struct Topic {
    name: String,
}

pub async fn new_topic(Json(req): Json<Topic>) -> (StatusCode, Json<serde_json::Value>) {
    let name = req.name;
    let no_spaces_name = name.replace(" ", "_");

    let new_topic = DB_Topic {
        name,
        no_spaces_name,
    };

    let is_created = create_topic(new_topic).await;

    match is_created {
        Ok(_) => (),
        Err(err) => match err {
            DbErr::Query(msg)
                if msg
                    .to_string()
                    .to_lowercase()
                    .contains("duplicate key value violates unique constraint") =>
            {
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({"Error": "Name must be unique"})),
                );
            }
            _ => {
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({"Error": "Unhandled DB error"})),
                );
            }
        },
    }

    // db::topics::create_topic();

    (StatusCode::OK, Json(json!("OK: Topic created succesfuly")))
}
