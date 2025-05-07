use axum::{
    routing::get,
    Router,
};
use crate::internal::ws::ws_handler::WsHandler;
use crate::internal::ws::socket_struct::AppState;
use crate::internal::ws::ws_handler_with_token::WsHandlerWithToken;

pub fn ws_router(app_state: AppState) -> Router {
    Router::new()
        .route("/ws", get(WsHandler::set_group_handler))
        .route("/ws/{group_id}/{table_number}", get(WsHandler::set_group_with_path_handler))
        .route("/ws/token", get(WsHandlerWithToken::set_group_handler_with_token))
        .route("/groups", get(WsHandler::list_groups_handler))
        .route("/health", get(WsHandler::health_check))
        .with_state(app_state)
}
