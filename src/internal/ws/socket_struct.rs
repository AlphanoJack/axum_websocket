use std::{collections::HashMap, sync::{Arc, Mutex}};

use serde::Deserialize;
use tokio::sync::broadcast;

// application state
#[derive(Clone)]
pub struct AppState {
    pub groups: Arc<Mutex<HashMap<String, Arc<broadcast::Sender<String>>>>>,
}

// ws handler struct
#[derive(Deserialize)]
pub struct WsQueryParams {
    pub group_id: String,
    pub table_number: u16,
    pub role: String,
}
