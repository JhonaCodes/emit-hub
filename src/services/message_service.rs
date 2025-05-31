use crate::models::message::{BroadcastMessage, BroadcastRequest, MessageSender, MessageType};
use crate::state::AppState;
use anyhow::Result;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct MessageService;

impl MessageService {
    pub async fn broadcast_message(
        state: Arc<AppState>,
        channel_id: Uuid,
        request: BroadcastRequest,
    ) -> Result<(BroadcastMessage, usize)> {
        // Verificar que el canal existe y está activo
        let channel = state.get_channel(&channel_id).await
            .ok_or_else(|| anyhow::anyhow!("Channel not found"))?;

        if !matches!(channel.status, crate::models::channel::ChannelStatus::Active) {
            anyhow::bail!("Channel is not active");
        }

        // Crear mensaje
        let message = BroadcastMessage {
            id: Uuid::new_v4(),
            channel_id,
            content: request.content,
            message_type: request.message_type.unwrap_or(MessageType::Broadcast),
            sender: MessageSender::Server,
            timestamp: Utc::now(),
        };

        // Persistir mensaje si está configurado
        if channel.settings.persist_messages {
            state.save_message(&message).await?;
        }

        // Crear respuesta WebSocket
        let ws_response = crate::models::message::WebSocketResponse {
            status: "broadcast".to_string(),
            message: message.content.clone(),
            channel_id: message.channel_id,
            timestamp: message.timestamp,
            data: None,
        };

        let json_message = serde_json::to_string(&ws_response)?;

        // Enviar a todos los clientes del canal
        let sent_count = state.broadcast_to_channel(&channel_id, &json_message).await?;

        tracing::info!(
            "Broadcasted message to {} clients in channel {} ({})",
            sent_count,
            channel.name,
            channel_id
        );

        Ok((message, sent_count))
    }
}