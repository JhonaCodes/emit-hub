use crate::models::channel::{Channel, ChannelSettings, ChannelStatus, CreateChannelRequest};
use crate::state::AppState;
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct ChannelService;

impl ChannelService {
    pub async fn create_channel(
        state: &AppState,
        request: CreateChannelRequest,
    ) -> Result<Channel> {
        let channel = Channel {
            id: Uuid::new_v4(),
            name: request.name,
            description: request.description,
            status: ChannelStatus::Created,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            settings: request.settings.unwrap_or_default(),
        };

        state.save_channel(&channel).await?;
        state.active_channels.write().await.insert(channel.id, channel.clone());

        tracing::info!("Created channel: {} ({})", channel.name, channel.id);
        Ok(channel)
    }

    pub async fn start_channel(state: &AppState, channel_id: Uuid) -> Result<Channel> {
        let mut active_channels = state.active_channels.write().await;

        if let Some(channel) = active_channels.get_mut(&channel_id) {
            channel.status = ChannelStatus::Active;
            channel.updated_at = Utc::now();
            state.save_channel(channel).await?;

            tracing::info!("Started channel: {} ({})", channel.name, channel.id);
            Ok(channel.clone())
        } else {
            anyhow::bail!("Channel not found")
        }
    }

    pub async fn pause_channel(state: &AppState, channel_id: Uuid) -> Result<Channel> {
        state.update_channel_status(channel_id, ChannelStatus::Paused).await?;

        if let Some(channel) = state.get_channel(&channel_id).await {
            tracing::info!("Paused channel: {} ({})", channel.name, channel.id);
            Ok(channel)
        } else {
            anyhow::bail!("Channel not found")
        }
    }

    pub async fn stop_channel(state: &AppState, channel_id: Uuid) -> Result<Channel> {
        state.update_channel_status(channel_id, ChannelStatus::Stopped).await?;

        // Cerrar todas las conexiones del canal
        {
            let mut connections = state.connections.lock().await;
            if let Some(sessions) = connections.remove(&channel_id) {
                for session in sessions {
                    let _ = session.close(None).await;
                }
            }
        }

        if let Some(channel) = state.get_channel(&channel_id).await {
            tracing::info!("Stopped channel: {} ({})", channel.name, channel.id);
            Ok(channel)
        } else {
            anyhow::bail!("Channel not found")
        }
    }

    pub async fn list_channels(state: &AppState) -> Vec<Channel> {
        state.active_channels.read().await.values().cloned().collect()
    }
}