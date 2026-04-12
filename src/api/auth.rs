use axum::Json;
use chrono;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

use crate::db::users::User;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    role: String,
}

pub async fn login(Json(req): Json<User>) {}

fn create_jwt(username: String, role: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(2))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: username,
        exp: expiration,
        role,
    };

    let secret = env::var("JWT_SECRET").unwrap(); //Env vars are checked on start of program, it's unlikely to fail

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
