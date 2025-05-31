use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: ChannelStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub settings: ChannelSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelStatus {
    Created,
    Active,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSettings {
    pub max_connections: usize,
    pub allow_client_messages: bool,
    pub persist_messages: bool,
    pub rate_limit_per_minute: Option<u32>,
}

impl Default for ChannelSettings {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            allow_client_messages: true,
            persist_messages: false,
            rate_limit_per_minute: Some(60),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub description: Option<String>,
    pub settings: Option<ChannelSettings>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateChannelStatusRequest {
    pub status: ChannelStatus,
}