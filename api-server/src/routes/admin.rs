use axum::routing::get;
use axum::{Json, Router};
use chrono::Utc;
use fred::interfaces::ClientLike;
use std::time::Instant;

use crate::errors::AppResult;
use crate::models::HealthResponse;
use crate::SharedState;

static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/version", get(version))
}

async fn health_check(
    state: axum::extract::State<SharedState>,
) -> AppResult<Json<HealthResponse>> {
    let db_status = match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => "healthy".to_string(),
        Err(e) => format!("unhealthy: {e}"),
    };

    let cache_status = match state.cache.ping::<()>(None).await {
        Ok(_) => "healthy".to_string(),
        Err(e) => format!("unhealthy: {e}"),
    };

    let vs_status = match state.vector_store.health_check().await {
        Ok(_) => "healthy".to_string(),
        Err(e) => format!("unhealthy: {e}"),
    };

    let uptime = START_TIME.get_or_init(Instant::now).elapsed().as_secs();

    Ok(Json(HealthResponse {
        status: if db_status == "healthy" && cache_status == "healthy" { "ok".into() } else { "degraded".into() },
        version: state.config.app.version.clone(),
        uptime_secs: uptime,
        database: db_status,
        cache: cache_status,
        vector_store: vs_status,
        timestamp: Utc::now(),
    }))
}

async fn version(
    state: axum::extract::State<SharedState>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": state.config.app.name,
        "version": state.config.app.version,
        "environment": state.config.app.environment,
    }))
}
