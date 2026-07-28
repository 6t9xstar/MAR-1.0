use axum::extract::{Multipart, Path, Query};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use uuid::Uuid;

use crate::errors::{AppError, AppResult};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::document::{DocumentQueryResult, DocumentSummary};
use crate::models::PaginationParams;
use crate::services::documents::DocumentService;
use crate::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/documents", get(list_documents))
        .route("/api/documents", post(upload_document))
        .route("/api/documents/{id}", get(get_document))
        .route("/api/documents/{id}", delete(delete_document))
        .route("/api/documents/{id}/query", post(query_document))
}

async fn list_documents(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    let (documents, total) = DocumentService::list_documents(
        &state.db, user.id, params.page, params.per_page,
    )
    .await?;

    Ok(Json(serde_json::json!({
        "data": documents,
        "page": params.page,
        "per_page": params.per_page,
        "total": total,
        "total_pages": total.div_ceil(params.per_page),
    })))
}

async fn upload_document(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    mut multipart: Multipart,
) -> AppResult<(axum::http::StatusCode, Json<DocumentSummary>)> {
    let mut file_data = None;
    let mut file_name = String::new();
    let mut conv_id: Option<Uuid> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::BadRequest(e.to_string()))? {
        match field.name() {
            Some("file") => {
                file_name = field.file_name().unwrap_or("unknown").to_string();
                file_data = Some(field.bytes().await.map_err(|e| AppError::BadRequest(e.to_string()))?);
            }
            Some("conversation_id") => {
                let val = field.text().await.unwrap_or_default();
                conv_id = Uuid::parse_str(&val).ok();
            }
            _ => {}
        }
    }

    let data = file_data.ok_or_else(|| AppError::BadRequest("No file provided".into()))?;

    let doc = DocumentService::upload(
        &state.db,
        &state.config.upload,
        user.id,
        conv_id,
        &file_name,
        &data,
    )
    .await?;

    Ok((axum::http::StatusCode::CREATED, Json(doc)))
}

async fn get_document(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<DocumentSummary>> {
    let doc = DocumentService::get_document(&state.db, id, user.id).await?;
    Ok(Json(DocumentSummary {
        id: doc.id,
        filename: doc.filename,
        original_filename: doc.original_filename,
        mime_type: doc.mime_type,
        size_bytes: doc.size_bytes,
        page_count: doc.page_count,
        status: doc.status,
        created_at: doc.created_at,
    }))
}

async fn delete_document(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    DocumentService::delete_document(&state.db, id, user.id).await?;
    Ok(Json(serde_json::json!({ "deleted": true })))
}

async fn query_document(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(body): Json<serde_json::Value>,
) -> AppResult<Json<Vec<DocumentQueryResult>>> {
    let query = body["query"].as_str()
        .ok_or_else(|| AppError::Validation("Missing 'query' parameter".into()))?;
    let max_results = body["max_results"].as_u64().unwrap_or(5) as u32;

    let results = DocumentService::query_document(&state.db, id, user.id, query, max_results).await?;
    Ok(Json(results))
}
