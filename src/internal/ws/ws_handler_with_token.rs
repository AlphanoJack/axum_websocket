use std::sync::Arc;

use axum::{extract::{ws::{Message, WebSocket}, Query, State, WebSocketUpgrade}, response::IntoResponse};
use futures::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc};

use crate::internal::{auth::jwt::{jwt_dto::WsJwtParams, jwt_token::{AuthData, JwtExtractor}}, ws::socket_struct::ServerMessage};

use super::socket_struct::AppState;

pub struct WsHandlerWithToken;

impl WsHandlerWithToken {
    // group handler using JWT and query params
    pub async fn set_group_handler_with_token(
        ws: WebSocketUpgrade,
        Query(params): Query<WsJwtParams>,
        State(state): State<AppState>,
        JwtExtractor(auth_data): JwtExtractor,
    ) -> impl IntoResponse {
        let tx = {
            let mut groups = state.groups.lock().unwrap();
            if !groups.contains_key(&params.group_id) {
                let (tx, _rx) = broadcast::channel::<String>(100);
                groups.insert(params.group_id.clone(), Arc::new(tx));
            }
            groups.get(&params.group_id).unwrap().clone()
        };

        tracing::info!(
            "authenticated client join to the group: {} (table: {})",
            params.group_id,
            auth_data.token_payload.table_number
        );

        // upgrade connection to websocket
        ws.on_upgrade(move |socket| Self::handle_socket_with_token(socket, tx, params, auth_data))
    }

    // authenticate user connection to websocket
    async fn handle_socket_with_token(
        socket: WebSocket,
        tx: Arc<broadcast::Sender<String>>,
        params: WsJwtParams,
        auth_data: AuthData,
    ) {
        let (mut sender, mut receiver) = socket.split();
        let (client_sender, mut client_receiver) = mpsc::channel::<Message>(100);
        let mut rx = tx.subscribe();

        // get table number from token payload
        let table_number = auth_data.token_payload.table_number.clone();

        // message filtering function
        fn should_receive_message(server_message: &ServerMessage, table_number: u16) -> bool {
            match &server_message.table_number {
                Some(numbers) => numbers.contains(&table_number),
                None => false,
            }
        }

        // parse message from server
        fn parse_server_message(message: &str) -> Option<ServerMessage> {
            serde_json::from_str(message).ok()
        }

        // send message to client from server
        let tx_clone = tx.clone();
        let table_number_clone = table_number.clone();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Some(server_msg) = parse_server_message(&msg) {
                    if should_receive_message(&server_msg, table_number_clone) {
                        if client_sender.send(Message::Text(msg.into())).await.is_err() {
                            break;
                        }
                    }
                } else {
                    if client_sender.send(Message::Text(msg.into())).await.is_err() {
                        break;
                    }
                }
            }
        });

        // client channer -> websocker (mpsc channel in tokio)
        tokio::spawn(async move {
            while let Some(message) = client_receiver.recv().await {
                if sender.send(message).await.is_err() {
                    break;
                }
            }
        });
        // 접속 메시지 전송
        let connect_msg = format!(
            "authenticated user {} (table {}) join to the {}",
            auth_data.token_payload.sub,
            table_number,
            params.group_id
        );
        let _ = tx.send(connect_msg);

        // 클라이언트로부터 메시지 수신 및 처리
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(text) => {
                    let formatted_msg = format!(
                        "[group: {}][table: {}][user: {}] {}",
                        params.group_id,
                        table_number,
                        auth_data.token_payload.sub,
                        text
                    );
                    if tx_clone.send(formatted_msg).is_err() {
                        break;
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }

        // 접속 종료 메시지 전송
        let disconnect_msg = format!(
            "authenticated user {} (table {}) leave the {}",
            auth_data.token_payload.sub,
            table_number,
            params.group_id
        );
        let _ = tx.send(disconnect_msg);

        tracing::info!(
            "authenticated user {} (table {}) leave the {}",
            auth_data.token_payload.sub,
            table_number,
            params.group_id
        );
    }
}
