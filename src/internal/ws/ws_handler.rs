
use std::sync::Arc;

use axum::{
    extract::{ws::{Message, WebSocket}, Path, Query, State, WebSocketUpgrade},
    response::IntoResponse};
use futures::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc};
use crate::internal::ws::socket_struct::ServerMessage;

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
        Self::set_websocket(ws, state, params).await
    }

    // grouping use path params
    pub async fn set_group_with_path_handler(
        ws: WebSocketUpgrade,
        Path(params): Path<WsQueryParams>,
        State(state): State<AppState>,
    ) -> impl IntoResponse {
        Self::set_websocket(ws, state, params).await
    }

    // set websocket and upgrade
    async fn set_websocket(
        ws: WebSocketUpgrade,
        state: AppState,
        params: WsQueryParams,
    ) -> impl IntoResponse {
        // get group channel or create new group channel
        let tx = {
            let mut groups = state.groups.lock().unwrap();
            if !groups.contains_key(&params.group_id) {
                let (tx, _rx) = broadcast::channel::<String>(100);
                groups.insert(params.group_id.clone(), Arc::new(tx));
            }
            groups.get(&params.group_id).unwrap().clone()
        };

        tracing::info!("client join to the group: {}", params.group_id);

        ws.on_upgrade(move |socket| Self::handle_socket(socket, tx, params))
    }

    // websocket connection handler
    async fn handle_socket(
        socket: WebSocket,
        tx: Arc<broadcast::Sender<String>>,
        params: WsQueryParams
    ) {
        // sperate receiver and sender
        let (mut sender, mut receiver) = socket.split();

        // client -> server message channel
        let (client_sender, mut client_receiver) = mpsc::channel::<Message>(100);

        // sbscribe to broadcast channel
        let mut rx = tx.subscribe();

        // message filtering function
        fn should_receive_message(server_message: &ServerMessage, params: &WsQueryParams) -> bool {
            let tablematch = match &server_message.table_number {
                Some(numbers) => numbers.contains(&params.table_number),
                None => true,
            };
            tablematch
        }

        // parse message from orderServer
        fn parse_server_message(message: &str) -> Option<ServerMessage> {
            serde_json::from_str(message).ok()
        }

        // send to message to client from server
        let params_clone = params.clone();
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                // 서버 메시지라면 JSON 파싱 후 필터링
                if let Some(server_msg) = parse_server_message(&msg) {
                    if should_receive_message(&server_msg, &params_clone) {
                        if client_sender.send(Message::Text(msg.into())).await.is_err() {
                            break;
                        }
                    }
                } else {
                    // 일반 메시지는 모두 전송
                    if client_sender.send(Message::Text(msg.into())).await.is_err() {
                        break;
                    }
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
        let connect_msg = format!(
            "tb {} join to the {}",
            params.table_number,
            params.group_id);
        let _ = tx.send(connect_msg);

        // 클라이언트로부터 메시지 수신 및 처리
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(text) => {
                    // 받은 메시지를 같은 그룹의 모든 클라이언트에게 브로드캐스트
                    let formatted_msg = format!(
                        "[group: {}][table: {}] {}",
                        params.group_id,
                        params.table_number,
                        text);
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
            "tb {} leave the {}",
            params.table_number,
            params.group_id);
        let _ = tx.send(disconnect_msg);

        tracing::info!("tb {} leave the {}",
            params.table_number,
            params.group_id);
    }

    // 서버 상태 확인용 핸들러
    pub async fn health_check() -> &'static str {
        "OK"
    }
}
