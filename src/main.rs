use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[derive(Serialize)]
struct MarketStatus {
    market_open: bool,
}

async fn market_open_status() -> Json<MarketStatus> {
    // 这里你可以添加逻辑来确定市场是否开盘
    let status = MarketStatus { market_open: true };
    Json(status)
}

#[tokio::main]
async fn main() {
    // 构建我们的应用程序并添加路由
    let app = Router::new()
        .route("/api/market/open", get(market_open_status))
        .layer(
            CorsLayer::new()
                .allow_origin(Any) // 允许任何来源的请求，你可以根据需要调整
                .allow_methods(Any),
        );

    // 设置监听地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));  // 改为3001端口
    println!("Listening on {}", addr);

    // 启动服务器
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
