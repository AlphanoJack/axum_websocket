use axum::{
    routing::post, Router
};

use crate::internal::{http_api::handler::message_handler, ws::socket_struct::AppState};

pub fn message_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/message", post(message_handler::MessageHandler::send_message_to_websocket))
        .with_state(app_state)
}
