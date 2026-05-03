use argon2::password_hash::rand_core::Error;
use axum::{
    Extension, Json,
    extract::State,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};
use chrono;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use jsonwebtoken::{TokenData, errors::ErrorKind as jErrKind};
use sea_orm::sqlx::decode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, sync::Arc};

use crate::api::users::{self, check_credentials};
use crate::db::users::User;

#[derive(Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    role: String,
}

pub async fn login(Json(req): Json<Login>) -> (StatusCode, Json<serde_json::Value>) {
    let username = req.username;
    let password = req.password;

    println!("Login");

    match check_credentials(&username, &password).await {
        Ok(c) => match c {
            true => {
                let jwt = if username == "admin" {
                    create_jwt(username, "admin".to_string())
                } else {
                    create_jwt(username, "user".to_string())
                };
                if jwt.is_err() {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            json!({"Error": format!("Something went wrong: {}", jwt.err().unwrap())}),
                        ),
                    );
                }

                (
                    StatusCode::OK,
                    Json(json!({
                        "Ok": "You succesfully logged in",
                        "JWT": jwt.unwrap(),
                    })),
                )
            }
            false => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"Error": "Wrong credentials"})),
            ),
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"Error": "Something went wrong"})),
        ),
    }

    // (
    //     StatusCode::INTERNAL_SERVER_ERROR,
    //     Json(json!({"Error": "Something went wrong"})),
    // )
}

fn create_jwt(username: String, role: String) -> Result<String, jsonwebtoken::errors::Error> {
    // 1. Get lifetime from env or default to 24 hours if missing
    let hours_str = env::var("JWT_LIFE_TIME").unwrap_or_else(|_| "2".to_string());
    let hours = hours_str.parse::<i64>().unwrap_or(24);

    // 2. Calculate expiration timestamp
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(hours))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username,
        exp: expiration,
        role,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

fn decode_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").unwrap(); //Env vars are checked on start of program, it's unlikely to fail

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}

fn construct_error(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({"Error": format!("{}", message)})),
    )
}

pub async fn check_jwt(
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok());

    if let Some(auth) = auth_header {
        let jwt_data = match decode_jwt(auth) {
            Ok(d) => d,
            Err(e) => match e.kind() {
                jErrKind::InvalidToken => {
                    return Err(construct_error("Invalid token"));
                }
                jErrKind::InvalidSignature => {
                    return Err(construct_error("Invalid signature"));
                }
                jErrKind::MissingRequiredClaim(_) => {
                    return Err(construct_error("Missing required claim"));
                }
                jErrKind::InvalidClaimFormat(_) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({"Error": "Invalid claim format"})),
                    ));
                }
                jErrKind::ExpiredSignature => {
                    return Err(construct_error("Token expired"));
                }
                _ => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"Error": "Failed to decode JWT"})),
                    ));
                }
            },
        };

        req.extensions_mut().insert(jwt_data.claims.sub);

        return Ok(next.run(req).await);
    }

    Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({ "Error": "Something went wrong" })),
    ))
}
