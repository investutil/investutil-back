use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use reqwest;
use scraper::{Html, Selector};

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
    // Build our application with routes
    let app = Router::new()
        .route("/api/market/open", get(market_open_status))
        .layer(
            CorsLayer::new()
                .allow_origin(Any) // Allow requests from any origin; adjust as needed
                .allow_methods(Any),
        );

    // Set the listening address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));  // Change to port 3001
    println!("Listening on {}", addr);

    // Run the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
