use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use axum::Json;
use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::db::{
    self,
    users::{User, create_user},
};

pub async fn new_user(Json(req): Json<User>) -> (StatusCode, Json<serde_json::Value>) {
    let username = req.username;
    let nickname = req.nickname;
    let password = req.password.as_bytes();

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
    };

    let is_created = create_user(new_user).await;
    if let Err(_) = is_created {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"Error": "Failed write user to db"})),
        );
    }

    (
        StatusCode::OK,
        Json(json!({"OK": "User succesfuly created"})),
    )
}
