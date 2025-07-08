use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::internal::ws::socket_struct::{AppState, ServerMessage};

pub struct MessageHandler;

impl MessageHandler {
    pub async fn send_message_to_websocket(
        State(state): State<AppState>,
        Json(payload): Json<ServerMessage>,
    ) -> impl IntoResponse {
        let groups = state.groups.lock().unwrap();

        match groups.get(&payload.group_id) {
            Some(tx) => {
                let message = serde_json::to_string(&payload).unwrap_or_default();
                match tx.send(message) {
                    Ok(_) => (StatusCode::OK, "Message sent successfully".to_string()),
                    Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to send message: {}", e)),
                }
            },
            None => (StatusCode::NOT_FOUND, format!("Group not found: {}", payload.group_id)),
        }
    }
}
