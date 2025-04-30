
use std::sync::Arc;

use axum::{
    extract::{ws::{Message, WebSocket}, Path, Query, State, WebSocketUpgrade},
    response::IntoResponse};
use futures::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc};
use super::socket_struct::{AppState, WsQueryParams};




pub struct WsHandler;

impl WsHandler {
    // serch group list handler
    pub async fn list_groups_handler(
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        let groups = state.groups.lock().unwrap();
        let group_list: Vec<String> = groups.keys().cloned().collect();
        format!("groups list: {:?}", group_list)
    }

    // grouping use query params
    pub async fn set_group_handler(
        ws: WebSocketUpgrade,
        Query(params): Query<WsQueryParams>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        let group_id = params.group_id;
        Self::set_websocket(ws, state, group_id).await
    }

    // grouping use path params
    pub async fn set_group_with_path_handler(
        ws: WebSocketUpgrade,
        Path(group_id): Path<String>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        Self::set_websocket(ws, state, group_id).await
    }

    // set websocket and upgrade
    async fn set_websocket(
        ws: WebSocketUpgrade,
        state: AppState,
        group_id: String,
    ) -> impl IntoResponse {
        // get group channel or create new group channel
        let tx = {
            let mut groups = state.groups.lock().unwrap();
            if !groups.contains_key(&group_id) {
                let (tx, _rx) = broadcast::channel::<String>(100);
                groups.insert(group_id.clone(), Arc::new(tx));
            }
            groups.get(&group_id).unwrap().clone()
        };

        tracing::info!("client join to the group: {}", group_id);

        ws.on_upgrade(move |socket| Self::handle_socket(socket, tx, group_id))
    }

    // websocket connection handler
    async fn handle_socket(
        socket: WebSocket,
        tx: Arc<broadcast::Sender<String>>,
        group_id: String
    ) {
        // sperate receiver and sender
        let (mut sender, mut receiver) = socket.split();

        // client -> server message channel
        let (client_sender, mut client_receiver) = mpsc::channel::<Message>(100);

        // sbscribe to broadcast channel
        let mut rx = tx.subscribe();

        // send to message to client
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                // send message to client from another client
                if client_sender.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
        });

        // client channel -> websocket
        tokio::spawn(async move {
            while let Some(message) = client_receiver.recv().await {
                if sender.send(message).await.is_err() {
                    break;
                }
            }
        });
        // 접속 메시지 전송
        let connect_msg = format!("새로운 사용자가 그룹 '{}' 에 접속했습니다.", group_id);
        let _ = tx.send(connect_msg);

        // 클라이언트로부터 메시지 수신 및 처리
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(text) => {
                    // 받은 메시지를 같은 그룹의 모든 클라이언트에게 브로드캐스트
                    let formatted_msg = format!("[{}] {}", group_id, text);
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
        let disconnect_msg = format!("사용자가 그룹 '{}' 에서 나갔습니다.", group_id);
        let _ = tx.send(disconnect_msg);

        tracing::info!("그룹 {} WebSocket 연결 종료", group_id);
    }

    // 서버 상태 확인용 핸들러
    pub async fn health_check() -> &'static str {
        "OK"
    }
}
