use crate::models::channel::{CreateChannelRequest};
use crate::models::message::BroadcastRequest;

use crate::state::AppState;
use actix_web::{get, post, put, web, HttpResponse, Result};

use uuid::Uuid;
use crate::services::channel_service::ChannelService;
use crate::services::message_service::MessageService;

#[post("/channels")]
async fn create_channel(
    state: web::Data<AppState>,
    request: web::Json<CreateChannelRequest>,
) -> Result<HttpResponse> {
    match ChannelService::create_channel(state.get_ref(), request.into_inner()).await {
        Ok(channel) => Ok(HttpResponse::Created().json(channel)),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("Error: {}", e))),
    }
}


#[get("/channels")]
async fn list_channels(state: web::Data<AppState>) -> Result<HttpResponse> {
    let channels = ChannelService::list_channels(state.get_ref()).await;
    Ok(HttpResponse::Ok().json(channels))
}

#[get("/channels/{channel_id}")]
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


#[put("/channels/{channel_id}/start")]
async fn start_channel(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let channel_id = path.into_inner();

    match ChannelService::start_channel(state.get_ref(), channel_id).await {
        Ok(channel) => Ok(HttpResponse::Ok().json(channel)),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("Error: {}", e))),
    }
}

#[put("/channels/{channel_id}/pause")]
async fn pause_channel(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let channel_id = path.into_inner();

    match ChannelService::pause_channel(state.get_ref(), channel_id).await {
        Ok(channel) => Ok(HttpResponse::Ok().json(channel)),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("Error: {}", e))),
    }
}


#[put("/channels/{channel_id}/stop")]
async fn stop_channel(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let channel_id = path.into_inner();

    match ChannelService::stop_channel(state.get_ref().into(), channel_id).await {
        Ok(channel) => Ok(HttpResponse::Ok().json(channel)),
        Err(e) => Ok(HttpResponse::BadRequest().json(format!("Error: {}", e))),
    }
}


#[post("/channels/{channel_id}/broadcast")]
async fn broadcast_message(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    request: web::Json<BroadcastRequest>,
) -> Result<HttpResponse> {
    let channel_id = path.into_inner();

    match MessageService::broadcast_message(
        state.get_ref(),
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