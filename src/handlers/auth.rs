use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{
    models::user::{CreateUserDto, LoginDto},
    services::auth::AuthService,
};

pub async fn register(
    State(auth_service): State<AuthService>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match auth_service.register(dto).await {
        Ok(user) => Ok(Json(json!({
            "message": "User registered successfully",
            "user": {
                "id": user.id,
                "email": user.email
            }
        }))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": e.to_string()
            })),
        )),
    }
}

pub async fn login(
    State(auth_service): State<AuthService>,
    Json(dto): Json<LoginDto>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    match auth_service.login(dto).await {
        Ok((token, user)) => Ok(Json(json!({
            "token": token,
            "user": {
                "id": user.id,
                "email": user.email
            }
        }))),
        Err(e) => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": e.to_string()
            })),
        )),
    }
} 