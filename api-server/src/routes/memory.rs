use axum::extract::{Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Json, Router};
use uuid::Uuid;

use crate::errors::AppResult;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::memory::{
    CreateMemoryRequest, MemoryEntry, MemorySearchResult, MemoryStats, UpdateMemoryRequest,
};
use crate::models::PaginationParams;
use crate::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/memory", get(list_memories))
        .route("/api/memory", post(create_memory))
        .route("/api/memory/search", post(search_memories))
        .route("/api/memory/stats", get(memory_stats))
        .route("/api/memory/{id}", get(get_memory))
        .route("/api/memory/{id}", put(update_memory))
        .route("/api/memory/{id}", delete(delete_memory))
        .route("/api/memory", delete(clear_memories))
}

async fn list_memories(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    let (memories, total) = state
        .memory
        .list_memories(user.id, params.page, params.per_page)
        .await?;

    Ok(Json(serde_json::json!({
        "data": memories,
        "page": params.page,
        "per_page": params.per_page,
        "total": total,
        "total_pages": total.div_ceil(params.per_page),
    })))
}

async fn create_memory(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Json(req): Json<CreateMemoryRequest>,
) -> AppResult<(axum::http::StatusCode, Json<MemoryEntry>)> {
    let memory = state.memory.create_memory(user.id, req).await?;
    Ok((axum::http::StatusCode::CREATED, Json(memory)))
}

async fn search_memories(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<Vec<MemorySearchResult>>> {
    let query = body["query"].as_str().unwrap_or("");
    let limit = body["limit"].as_u64().unwrap_or(10) as u32;
    let min_importance = body["min_importance"].as_u64().unwrap_or(0) as u8;

    let results = state
        .memory
        .search_memories(user.id, query, None, limit, min_importance)
        .await?;

    Ok(Json(results))
}

async fn memory_stats(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
) -> AppResult<Json<MemoryStats>> {
    let stats = state.memory.get_stats(user.id).await?;
    Ok(Json(stats))
}

async fn get_memory(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<MemoryEntry>> {
    let memory = state.memory.get_memory(user.id, id).await?;
    Ok(Json(memory))
}

async fn update_memory(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateMemoryRequest>,
) -> AppResult<Json<MemoryEntry>> {
    let memory = state.memory.update_memory(user.id, id, req).await?;
    Ok(Json(memory))
}

async fn delete_memory(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    state.memory.delete_memory(user.id, id).await?;
    Ok(Json(serde_json::json!({ "deleted": true })))
}

async fn clear_memories(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
) -> AppResult<Json<serde_json::Value>> {
    state.memory.clear_memories(user.id).await?;
    Ok(Json(serde_json::json!({ "cleared": true })))
}
