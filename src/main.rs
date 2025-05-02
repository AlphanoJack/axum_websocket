mod config;
mod internal;
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};

use axum::{routing::get, Router};
use internal::ws::{socket_struct::AppState, ws_handler::WsHandler};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Level;


#[tokio::main]
async fn main() {
    // setup logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    // set app state
    let app_state = AppState {
        groups: Arc::new(Mutex::new(HashMap::new())),
    };

    // set router
    let app = Router::new()
        .route("/ws", get(WsHandler::set_group_handler))
        .route("/ws/{group_id}/{table_number}/{role}", get(WsHandler::set_group_with_path_handler))
        .route("/groups", get(WsHandler::list_groups_handler))
        .route("/health", get(WsHandler::health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::info!("서버 시작: {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    // load config
    // let config = Config::init_config();
    // tracing::info!("Server is running on port {}", config.server_port);
}
