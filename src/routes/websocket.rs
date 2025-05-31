use crate::models::message::{WebSocketResponse, MessageType, MessageSender, BroadcastMessage};
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse, Result, Error};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/channels/{channel_id}/ws", web::get().to(websocket_handler));
}

async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let channel_id = path.into_inner();

    // Verificar que el canal existe y está activo
    let channel = match state.get_channel(&channel_id).await {
        Some(channel) => channel,
        None => {
            let error_response = WebSocketResponse {
                status: "error".to_string(),
                message: "Channel not found".to_string(),
                channel_id,
                timestamp: Utc::now(),
                data: None,
            };
            return Ok(HttpResponse::NotFound().json(error_response));
        }
    };

    if !matches!(channel.status, crate::models::channel::ChannelStatus::Active) {
        let error_response = WebSocketResponse {
            status: "denied".to_string(),
            message: format!("Channel {} is not active", channel.name),
            channel_id,
            timestamp: Utc::now(),
            data: None,
        };
        return Ok(HttpResponse::Forbidden().json(error_response));
    }

    // Establecer conexión WebSocket
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    // Agregar conexión al estado
    state.add_connection(channel_id, session.clone()).await;

    // Enviar mensaje de bienvenida
    let welcome_msg = WebSocketResponse {
        status: "connected".to_string(),
        message: format!("Connected to channel: {}", channel.name),
        channel_id,
        timestamp: Utc::now(),
        data: Some(serde_json::json!({
            "channel": channel,
            "connection_id": Uuid::new_v4()
        })),
    };

    let welcome_json = serde_json::to_string(&welcome_msg).unwrap();
    let _ = session.text(welcome_json).await;

    // Procesar mensajes entrantes
    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    let state_clone: Arc<AppState> = state.get_ref().clone().into();
    let mut session_clone = session.clone();

    actix_web::rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    tracing::debug!("Received message in channel {}: {}", channel_id, text);

                    // Verificar si el canal permite mensajes de clientes
                    if let Some(current_channel) = state_clone.get_channel(&channel_id).await {
                        if current_channel.settings.allow_client_messages {
                            // Crear mensaje del cliente
                            let client_message = BroadcastMessage {
                                id: Uuid::new_v4(),
                                channel_id,
                                content: text.to_string(),
                                message_type: MessageType::ClientMessage,
                                sender: MessageSender::Client("anonymous".to_string()),
                                timestamp: Utc::now(),
                            };

                            // Persistir si está configurado
                            if current_channel.settings.persist_messages {
                                let _ = state_clone.save_message(&client_message).await;
                            }

                            // Reenviar a todos los clientes del canal
                            let response = WebSocketResponse {
                                status: "client_message".to_string(),
                                message: format!("Client message in {}: {}", current_channel.name, text),
                                channel_id,
                                timestamp: client_message.timestamp,
                                data: Some(serde_json::json!({
                                    "original_message": *text,
                                    "sender": "client"
                                })),
                            };

                            let json_response = serde_json::to_string(&response).unwrap();
                            let _ = state_clone.broadcast_to_channel(&channel_id, &json_response).await;
                        }
                    }
                }
                Ok(AggregatedMessage::Ping(msg)) => {
                    let _ = session_clone.pong(&msg).await;
                }
                Ok(AggregatedMessage::Close(_)) => {
                    tracing::debug!("WebSocket connection closed for channel {}", channel_id);
                    break;
                }
                _ => {}
            }
        }

        // Limpiar conexión
        state_clone.remove_connection(&channel_id, &session_clone).await;
        tracing::debug!("Cleaned up connection for channel {}", channel_id);
    });

    Ok(res)
}