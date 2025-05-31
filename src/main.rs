// src/main.rs
use actix_web::{web, App, HttpServer, middleware::Logger};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod routes;
mod services;
mod state;
mod config;

use config::Config;
use state::AppState;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // 🔧 Configurar logging con colores y niveles
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "emit_hub=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 📋 Banner de inicio
    println!("🚀 EmitHub - Real-time Broadcasting Service");
    println!("   Version: {}", env!("CARGO_PKG_VERSION"));
    println!("   Repository: https://github.com/your-org/emit-hub\n");

    // ⚙️ Cargar configuración desde variables de entorno
    let config = Config::from_env()?;
    tracing::info!("📡 Starting EmitHub on {}:{}", config.host, config.port);
    tracing::info!("💾 Database: {}", config.db_path);
    tracing::info!("🔌 Max connections per channel: {}", config.max_connections_per_channel);

    // 🗃️ Inicializar estado de la aplicación con ReDB
    tracing::info!("🔄 Initializing database and loading active channels...");
    let app_state = Arc::new(AppState::new(&config.db_path).await?);
    tracing::info!("✅ Application state initialized successfully");

    // 🌐 Crear y configurar servidor HTTP
    let server_host = config.host.clone();
    let server_port = config.port;

    HttpServer::new(move || {
        App::new()
            // 📊 Inyectar estado compartido
            .app_data(web::Data::from(app_state.clone()))

            // 📝 Middleware de logging de requests
            .wrap(Logger::default())

            // 🛣️ Configurar rutas principales
            .service(
                web::scope("/api/v1")
                    // 📡 Gestión de canales
                    .configure(routes::channel::configure)

                    // 🔌 WebSocket connections
                    .configure(routes::websocket::configure)

                    // ❤️ Health checks
                    .configure(routes::health::configure)
            )

            // 📄 Ruta de información general
            .route("/", web::get().to(root_handler))

            // 📚 Ruta de documentación de la API
            .route("/docs", web::get().to(docs_handler))
    })
        .bind((server_host, server_port))?
        .run()
        .await?;

    Ok(())
}

// 🏠 Handler para la ruta raíz
async fn root_handler() -> actix_web::Result<actix_web::HttpResponse> {
    let info = serde_json::json!({
        "service": "EmitHub",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Real-time broadcasting microservice",
        "endpoints": {
            "channels": "/api/v1/channels",
            "websocket": "/api/v1/channels/{id}/ws",
            "health": "/api/v1/health",
            "docs": "/docs"
        },
        "status": "running",
        "timestamp": chrono::Utc::now()
    });

    Ok(actix_web::HttpResponse::Ok().json(info))
}

// 📚 Handler para documentación de la API
async fn docs_handler() -> actix_web::Result<actix_web::HttpResponse> {
    let docs = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <title>EmitHub API Documentation</title>
        <style>
            body { font-family: Arial, sans-serif; margin: 40px; }
            .endpoint { background: #f5f5f5; padding: 15px; margin: 10px 0; border-radius: 5px; }
            .method { color: white; padding: 4px 8px; border-radius: 3px; font-weight: bold; }
            .post { background: #28a745; }
            .get { background: #007bff; }
            .put { background: #ffc107; color: black; }
            .delete { background: #dc3545; }
            .websocket { background: #6f42c1; }
            pre { background: #f8f9fa; padding: 10px; border-radius: 3px; overflow-x: auto; }
        </style>
    </head>
    <body>
        <h1>🚀 EmitHub API Documentation</h1>
        <p>Real-time broadcasting microservice built with Rust and Actix-web</p>
        
        <h2>📡 Channel Management</h2>
        
        <div class="endpoint">
            <span class="method post">POST</span> <code>/api/v1/channels</code>
            <p>Create a new broadcast channel</p>
            <pre>{
  "name": "Channel Name",
  "description": "Optional description",
  "settings": {
    "max_connections": 1000,
    "allow_client_messages": true,
    "persist_messages": false,
    "rate_limit_per_minute": 60
  }
}</pre>
        </div>
        
        <div class="endpoint">
            <span class="method get">GET</span> <code>/api/v1/channels</code>
            <p>List all channels</p>
        </div>
        
        <div class="endpoint">
            <span class="method get">GET</span> <code>/api/v1/channels/{id}</code>
            <p>Get specific channel information</p>
        </div>
        
        <div class="endpoint">
            <span class="method put">PUT</span> <code>/api/v1/channels/{id}/start</code>
            <p>Start channel emission (allow WebSocket connections)</p>
        </div>
        
        <div class="endpoint">
            <span class="method put">PUT</span> <code>/api/v1/channels/{id}/pause</code>
            <p>Pause channel emission</p>
        </div>
        
        <div class="endpoint">
            <span class="method put">PUT</span> <code>/api/v1/channels/{id}/stop</code>
            <p>Stop channel emission and close all connections</p>
        </div>
        
        <div class="endpoint">
            <span class="method post">POST</span> <code>/api/v1/channels/{id}/broadcast</code>
            <p>Broadcast message to all connected clients</p>
            <pre>{
  "content": "Your message here",
  "message_type": "Broadcast"
}</pre>
        </div>
        
        <h2>🔌 WebSocket Connection</h2>
        
        <div class="endpoint">
            <span class="method websocket">WS</span> <code>/api/v1/channels/{id}/ws</code>
            <p>Connect to channel via WebSocket for real-time messages</p>
        </div>
        
        <h2>❤️ Health & Monitoring</h2>
        
        <div class="endpoint">
            <span class="method get">GET</span> <code>/api/v1/health</code>
            <p>Service health check</p>
        </div>
        
        <div class="endpoint">
            <span class="method get">GET</span> <code>/api/v1/ready</code>
            <p>Service readiness check</p>
        </div>
        
        <h2>🔧 Environment Variables</h2>
        <ul>
            <li><code>HOST</code> - Server host (default: 127.0.0.1)</li>
            <li><code>PORT</code> - Server port (default: 8080)</li>
            <li><code>DB_PATH</code> - ReDB database file path (default: emit_hub.redb)</li>
            <li><code>MAX_CONNECTIONS</code> - Max connections per channel (default: 1000)</li>
            <li><code>MESSAGE_SIZE_LIMIT</code> - Max message size in bytes (default: 1MB)</li>
        </ul>
        
        <h2>🚀 Quick Start</h2>
        <pre>
# 1. Create a channel
curl -X POST http://localhost:8080/api/v1/channels \
  -H "Content-Type: application/json" \
  -d '{"name": "Test Channel"}'

# 2. Start the channel (use returned UUID)
curl -X PUT http://localhost:8080/api/v1/channels/{channel_id}/start

# 3. Connect via WebSocket
wscat -c ws://localhost:8080/api/v1/channels/{channel_id}/ws

# 4. Broadcast a message
curl -X POST http://localhost:8080/api/v1/channels/{channel_id}/broadcast \
  -H "Content-Type: application/json" \
  -d '{"content": "Hello World!", "message_type": "Broadcast"}'
        </pre>
    </body>
    </html>
    "#;

    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/html")
        .body(docs))
}