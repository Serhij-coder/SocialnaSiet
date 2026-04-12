use std::env;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::Json;
use axum::http::StatusCode;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct TestUser {
    username: String,
}

use crate::db;
use crate::db::users::{User, is_user};
use crate::image::save_image;

pub async fn new_public_user(Json(req): Json<User>) -> (StatusCode, Json<serde_json::Value>) {
    create_user(Json(req), "user".to_string()).await
}

async fn create_user(Json(req): Json<User>, role: String) -> (StatusCode, Json<serde_json::Value>) {
    let username = req.username;
    let nickname = req.nickname;
    let password = req.password;

    if let Err(message) = check_data_correctness(&nickname, &username, &password) {
        return message;
    }

    let mut profile_picture = if !req.profile_picture.is_empty() {
        match save_image(req.profile_picture).await {
            Ok(str) => str,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"Error": format!("{}", e)})),
                );
            }
        }
    } else {
        "pfp".to_string()
    };

    if role.as_bytes() == b"admin" {
        profile_picture = "admin".to_string()
    };

    let password = password.as_bytes();

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt);
    let password_hash = if let Err(_) = password_hash {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"Error": "Password error"})),
        );
    } else {
        password_hash.unwrap().to_string()
    };

    // Control new hash
    let parsed_hash = PasswordHash::new(&password_hash);
    if let Err(_) = parsed_hash {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"Error": "Password error"})),
        );
    };

    // Create user and write to db
    let new_user = User {
        username,
        password: password_hash,
        nickname,
        profile_picture,
        role,
    };

    let is_created = db::users::create_user(new_user).await;

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
                    Json(json!({"Error": "Username must be unique"})),
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

    (
        StatusCode::OK,
        Json(json!({"OK": "User succesfuly created"})),
    )
}

pub async fn check_username_availability(
    Json(req): Json<TestUser>,
) -> (StatusCode, Json<serde_json::Value>) {
    let username = req.username;

    let message = format!("Username is alredy taken for username {username}");

    let is_user = is_user(username.as_ref()).await;

    match is_user {
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"Error": "Failed to read db"})),
        ),
        Ok(true) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"Error": message})),
        ),
        Ok(false) => (StatusCode::OK, Json(json!({"OK": "Username is available"}))),
    }
}

pub async fn init_admin_user() -> Result<String, ()> {
    if let Ok(true) = is_user("admin").await {
        Ok("Admin user alredy exist :)".to_string())
    } else {
        let user = User {
            username: "admin".to_string(),
            password: env::var("ADMIN_PASSWORD").unwrap(),
            nickname: "Admin".to_string(),
            role: "".to_string(),
            profile_picture: "".to_string(),
        };
        let (status, response) = create_user(Json(user), "admin".to_string()).await;
        if status == StatusCode::OK {
            Ok("Successfully created admin user".to_string())
        } else {
            // Log the error from the JSON if needed
            println!("Failed to create admin: {:?}", response);
            Err(())
        }
    }
}

fn check_data_correctness(
    nickname: &str,
    username: &str,
    password: &str,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    // Basic bad requests
    if nickname.trim() == "" || username.trim() == "" || password.trim() == "" {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"Error": "Nickname or username or password can't be empty"})),
        ));
    }
    if username.trim().contains(" ") {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"Error": "Username can't contain spaces"})),
        ));
    }
    if password.trim().contains(" ") {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"Error": "Password can't contain spaces"})),
        ));
    }

    // Lengths control
    if username.len() < 5 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"Error": "Username length must be minimum 5 symbols"})),
        ));
    }
    if password.len() < 5 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"Error": "Password length must be minimum 5 symbols"})),
        ));
    }

    Ok(())
}
