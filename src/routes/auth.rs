use axum::{
    routing::post,
    Router,
};

use crate::handlers::auth::{login, register};
use crate::services::auth::AuthService;

pub fn auth_routes(auth_service: AuthService) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(auth_service)
} 