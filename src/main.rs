use actix_web::{web, App, HttpServer, middleware::Logger};
use std::sync::Arc;

mod models;
mod state;
mod config;
mod routes;
mod services;

// Importar las funciones que ya tienes
use crate::state::AppState;
use crate::config::Config;
use crate::routes::channel::{broadcast_message, create_channel, get_channel, list_channels, pause_channel, start_channel, stop_channel};
use crate::routes::websocket::{logs_handler, websocket_handler};
// Aquí pon todas las funciones de routes que ya tienes:
// ws_handler, emit_to_channel, start_emission, stop_emission, etc.

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // 📊 Setup de logging con colores y timestamps
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp_secs()
        .init();

    let config = Config::from_env()?;
    let app_state = Arc::new(AppState::new(&config.db_path).await?);

    log::info!("🚀 Starting EmitHub server on {}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(app_state.clone()))
            .wrap(Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T"))
            .service(logs_handler)
            .service(
                web::scope("/api/v1")
                    .service(websocket_handler)
                    .service(create_channel)
                    .service(list_channels)
                    .service(get_channel)
                    .service(start_channel)
                    .service(pause_channel)
                    .service(stop_channel)
                    .service(broadcast_message)
            )
    })
        .bind((config.host, config.port))?
        .run()
        .await?;

    Ok(())
}