use crate::models::channel::{CreateChannelRequest, UpdateChannelStatusRequest};
use crate::models::message::BroadcastRequest;

use crate::state::AppState;
use actix_web::{web, HttpResponse, Result};
use std::sync::Arc;
use uuid::Uuid;
use crate::services::channel_service::ChannelService;
use crate::services::message_service::MessageService;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/channels")
            .route("", web::post().to(create_channel))
            .route("", web::get().to(list_channels))
            .route("/{channel_id}", web::get().to(get_channel))
            .route("/{channel_id}/start", web::put().to(start_channel))
            .route("/{channel_id}/pause", web::put().to(pause_channel))
            .route("/{channel_id}/stop", web::put().to(stop_channel))
            .route("/{channel_id}/broadcast", web::post().to(broadcast_message)),
    );
}

async fn create_channel(
    state: web::Data<AppState>,
    request: web::Json<CreateChannelRequest>,
) -> Result<HttpResponse> {
    match ChannelService::create_channel(state.get_ref().clone().into(), request.into_inner()).await {
        Ok(channel) => Ok(HttpResponse::Created().json(channel)),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("Error: {}", e))),
    }
}

async fn list_channels(state: web::Data<AppState>) -> Result<HttpResponse> {
    let channels = ChannelService::list_channels(state.get_ref().clone().into()).await;
    Ok(HttpResponse::Ok().json(channels))
}

async fn get_channel(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let channel_id = path.into_inner();

    match state.get_channel(&channel_id).await {
        Some(channel) => Ok(HttpResponse::Ok().json(channel)),
        None => Ok(HttpResponse::NotFound().json("Channel not found")),
    }
}

async fn start_channel(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let channel_id = path.into_inner();

    match ChannelService::start_channel(state.get_ref().clone().into(), channel_id).await {
        Ok(channel) => Ok(HttpResponse::Ok().json(channel)),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("Error: {}", e))),
    }
}

async fn pause_channel(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let channel_id = path.into_inner();

    match ChannelService::pause_channel(state.get_ref().clone().into(), channel_id).await {
        Ok(channel) => Ok(HttpResponse::Ok().json(channel)),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("Error: {}", e))),
    }
}

async fn stop_channel(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let channel_id = path.into_inner();

    match ChannelService::stop_channel(state.get_ref().clone().into(), channel_id).await {
        Ok(channel) => Ok(HttpResponse::Ok().json(channel)),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("Error: {}", e))),
    }
}

async fn broadcast_message(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    request: web::Json<BroadcastRequest>,
) -> Result<HttpResponse> {
    let channel_id = path.into_inner();

    match MessageService::broadcast_message(
        state.get_ref().clone().into(),
        channel_id,
        request.into_inner(),
    ).await {
        Ok((message, sent_count)) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "message": message,
            "sent_to": sent_count,
            "status": "success"
        }))),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("Error: {}", e))),
    }
}