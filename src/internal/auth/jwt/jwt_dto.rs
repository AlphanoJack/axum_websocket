use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPayload {
    pub sub: String,
    #[serde(rename = "restaurantName")]
    pub restaurant_name: String,
    #[serde(rename = "tableNumber")]
    pub table_number: u16,
    #[serde(rename = "tableCount")]
    pub table_count: u16,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct WsJwtParams {
    pub group_id: String,
}

