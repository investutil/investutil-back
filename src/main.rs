use std::net::SocketAddr;
use axum::{
    routing::get,
    Router,
    Json,
    http::{HeaderValue, Method},
};
use tower_http::cors::{CorsLayer, Any};
use dotenvy::dotenv;
use std::env;
use reqwest;
use scraper::{Html, Selector};
use serde::Serialize;
use tokio::net::TcpListener;
use axum::http::header;

mod models;
mod services;
mod handlers;
mod routes;

use crate::routes::auth::auth_routes;
use crate::services::auth::AuthService;

#[derive(Serialize)]
struct MarketStatus {
    market_open: bool,
}

async fn market_open_status() -> Json<MarketStatus> {
    let market_open = check_market_open().await;
    Json(MarketStatus { market_open })
}

async fn check_market_open() -> bool {
    let url = "https://finance.yahoo.com/quote/%5EGSPC/";
    let client = reqwest::Client::new();
    let res = client.get(url)
        .send()
        .await
        .expect("Failed to fetch data")
        .text()
        .await
        .expect("Failed to read response text");

    let document = Html::parse_document(&res);
    let selector = Selector::parse("div[slot='marketTimeNotice'] span").unwrap();
    
    if let Some(element) = document.select(&selector).next() {
        let market_status = element.inner_html();
        return market_status.contains("Market Open");
    }

    false
}

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Database connection
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // JWT secret
    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    // Create auth service
    let auth_service = AuthService::new(pool.clone(), jwt_secret);

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_credentials(true)
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
        ]);

    // Build our application with routes
    let app = Router::new()
        .route("/api/market/open", get(market_open_status))
        .nest("/auth", auth_routes(auth_service))
        .layer(cors);

    // Get host and port from environment
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();
    println!("Server running on http://{}", addr);

    // Create and bind the TCP listener
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on http://{}", addr);

    // Run the server
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
