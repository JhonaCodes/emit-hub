use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastMessage {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub content: String,
    pub message_type: MessageType,
    pub sender: MessageSender,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Broadcast,
    System,
    ClientMessage,
    StatusUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageSender {
    Server,
    Client(String),
    System,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastRequest {
    pub content: String,
    pub message_type: Option<MessageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketResponse {
    pub status: String,
    pub message: String,
    pub channel_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub data: Option<serde_json::Value>,
}