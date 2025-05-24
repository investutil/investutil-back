use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
};
use crate::application::{
    services::auth_service::AuthService,
    dtos::auth::{LoginDto, RegisterDto, AuthResponse},
};
use std::sync::Arc;

pub fn auth_routes(auth_service: Arc<AuthService>) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(auth_service)
}

async fn register(
    State(auth_service): State<Arc<AuthService>>,
    Json(dto): Json<RegisterDto>,
) -> Result<Json<AuthResponse>, String> {
    auth_service
        .register(dto)
        .await
        .map(|_| Json(AuthResponse { token: "Registration successful".to_string() }))
        .map_err(|e| e.to_string())
}

async fn login(
    State(auth_service): State<Arc<AuthService>>,
    Json(dto): Json<LoginDto>,
) -> Result<Json<AuthResponse>, String> {
    auth_service
        .login(dto)
        .await
        .map(|token| Json(AuthResponse { token }))
        .map_err(|e| e.to_string())
} 