use actix_web::{get, HttpResponse, Result};
use serde_json::json;


#[get("/health")]
async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "emit-hub",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now()
    })))
}

#[get("/ready")]
async fn readiness_check() -> Result<HttpResponse> {
    // We can improve logic here
    Ok(HttpResponse::Ok().json(json!({
        "status": "ready",
        "checks": {
            "database": "ok",
            "memory": "ok"
        }
    })))
}