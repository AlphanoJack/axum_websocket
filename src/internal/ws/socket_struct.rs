use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use serde_json::Value;

// application state
#[derive(Clone)]
pub struct AppState {
    pub groups: Arc<Mutex<HashMap<String, Arc<broadcast::Sender<String>>>>>,
}

// ws handler struct
#[derive(Deserialize, Clone)]
pub struct WsQueryParams {
    pub group_id: String,
    pub table_number: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerMessage {
    pub group_id: String,
    pub table_number: Option<Vec<u16>>,
    pub message_type: String,
    pub payload: Value,
}
