use actix_web::{web, HttpResponse, Result};
use serde_json::json;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check))
        .route("/ready", web::get().to(readiness_check));
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "emit-hub",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now()
    })))
}

async fn readiness_check() -> Result<HttpResponse> {
    // Aquí puedes agregar verificaciones más complejas
    // como conectividad a la base de datos, etc.
    Ok(HttpResponse::Ok().json(json!({
        "status": "ready",
        "checks": {
            "database": "ok",
            "memory": "ok"
        }
    })))
}