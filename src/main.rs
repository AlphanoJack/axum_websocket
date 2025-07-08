mod config;
mod internal;
mod routes;
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};

use axum::Router;
use internal::ws::socket_struct::AppState;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Level;

use crate::routes::{ws_router, message_router};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // setup logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    // set app state
    let app_state = AppState {
        groups: Arc::new(Mutex::new(HashMap::new())),
    };

    let ws_router = ws_router(app_state.clone());
    let message_router = message_router(app_state.clone());

    // set router
    let app = Router::new()
        .merge(ws_router)
        .merge(message_router)
        .layer(TraceLayer::new_for_http());


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::info!("서버 시작: {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    // load config
    // let config = Config::init_config();
    // tracing::info!("Server is running on port {}", config.server_port);
}
