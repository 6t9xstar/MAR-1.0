use axum::extract::{Path, Query};
use axum::response::{sse::Event, Sse};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use futures::stream::Stream;
use futures::StreamExt;
use std::convert::Infallible;
use std::time::Duration;

use crate::errors::AppResult;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::chat::{
    ConversationSummary, CreateConversationRequest,
    SendMessageRequest, UpdateConversationRequest,
};
use crate::models::PaginationParams;
use crate::services::chat::ChatService;
use crate::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/api/conversations", get(list_conversations))
        .route("/api/conversations", post(create_conversation))
        .route("/api/conversations/{id}", get(get_conversation))
        .route("/api/conversations/{id}", post(update_conversation))
        .route("/api/conversations/{id}", delete(delete_conversation))
        .route("/api/conversations/{id}/messages", get(get_messages))
        .route("/api/chat", post(send_message))
        .route("/api/chat/stream", post(stream_message))
}

#[utoipa::path(
    get,
    path = "/api/conversations",
    responses((status = 200, body = Vec<ConversationSummary>)),
    tag = "Chat"
)]
async fn list_conversations(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    let (conversations, total) = ChatService::get_conversations(
        &state.db, user.id, params.page, params.per_page,
    )
    .await?;

    Ok(Json(serde_json::json!({
        "data": conversations,
        "page": params.page,
        "per_page": params.per_page,
        "total": total,
        "total_pages": total.div_ceil(params.per_page),
    })))
}

#[utoipa::path(
    post,
    path = "/api/conversations",
    request_body = CreateConversationRequest,
    responses((status = 201, body = ConversationSummary)),
    tag = "Chat"
)]
async fn create_conversation(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Json(req): Json<CreateConversationRequest>,
) -> AppResult<(axum::http::StatusCode, Json<serde_json::Value>)> {
    let conv = ChatService::create_conversation(
        &state.db, user.id, req.title, req.language,
    )
    .await?;

    Ok((
        axum::http::StatusCode::CREATED,
        Json(serde_json::json!(conv)),
    ))
}

#[utoipa::path(
    get,
    path = "/api/conversations/{id}",
    responses((status = 200, body = ConversationSummary)),
    tag = "Chat"
)]
async fn get_conversation(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Path(id): Path<uuid::Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    let conv = ChatService::get_conversation(&state.db, user.id, id).await?;
    Ok(Json(serde_json::json!(conv)))
}

#[utoipa::path(
    put,
    path = "/api/conversations/{id}",
    request_body = UpdateConversationRequest,
    tag = "Chat"
)]
async fn update_conversation(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Path(id): Path<uuid::Uuid>,
    Json(req): Json<UpdateConversationRequest>,
) -> AppResult<Json<serde_json::Value>> {
    if let Some(title) = req.title {
        sqlx::query("UPDATE conversations SET title = $1, updated_at = $2 WHERE id = $3 AND user_id = $4")
            .bind(&title)
            .bind(chrono::Utc::now())
            .bind(id)
            .bind(user.id)
            .execute(&state.db)
            .await?;
    }

    if let Some(archived) = req.is_archived {
        sqlx::query("UPDATE conversations SET is_archived = $1, updated_at = $2 WHERE id = $3 AND user_id = $4")
            .bind(archived)
            .bind(chrono::Utc::now())
            .bind(id)
            .bind(user.id)
            .execute(&state.db)
            .await?;
    }

    let conv = ChatService::get_conversation(&state.db, user.id, id).await?;
    Ok(Json(serde_json::json!(conv)))
}

#[utoipa::path(
    delete,
    path = "/api/conversations/{id}",
    tag = "Chat"
)]
async fn delete_conversation(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Path(id): Path<uuid::Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    sqlx::query("DELETE FROM messages WHERE conversation_id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    sqlx::query("DELETE FROM conversations WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user.id)
        .execute(&state.db)
        .await?;

    Ok(Json(serde_json::json!({ "deleted": true })))
}

#[utoipa::path(
    get,
    path = "/api/conversations/{id}/messages",
    tag = "Chat"
)]
async fn get_messages(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Path(conv_id): Path<uuid::Uuid>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<serde_json::Value>> {
    let messages = ChatService::get_messages(
        &state.db, user.id, conv_id, params.page, params.per_page,
    )
    .await?;

    Ok(Json(serde_json::json!({ "data": messages })))
}

#[utoipa::path(
    post,
    path = "/api/chat",
    request_body = SendMessageRequest,
    tag = "Chat"
)]
async fn send_message(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Json(req): Json<SendMessageRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let (message, conv_id) = ChatService::send_message(&state, user.id, req).await?;
    Ok(Json(serde_json::json!({
        "message": message,
        "conversation_id": conv_id,
    })))
}

#[utoipa::path(
    post,
    path = "/api/chat/stream",
    request_body = SendMessageRequest,
    tag = "Chat"
)]
async fn stream_message(
    state: axum::extract::State<SharedState>,
    user: AuthenticatedUser,
    Json(req): Json<SendMessageRequest>,
) -> AppResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let conv_id = match req.conversation_id {
        Some(id) => {
            ChatService::get_conversation(&state.db, user.id, id).await?;
            id
        }
        None => {
            let conv = ChatService::create_conversation(
                &state.db, user.id, None, req.language.clone(),
            )
            .await?;
            conv.id
        }
    };

    let messages = ChatService::get_messages(&state.db, user.id, conv_id, 1, 1000).await?;

    let language = state.nlp.detect_language(&req.content);
    let domain = state.knowledge.domain_for_text(&req.content, &format!("{:?}", language));
    let system_prompt = state.prompt_builder.build(
        &language,
        &domain,
        Some(&state.knowledge),
        None,
        &req.content,
    );

    let inference = state.0.clone();
    let stream = inference.inference
        .chat_stream(messages, Some(system_prompt.clone()), req.temperature, req.max_tokens)
        .await?;

    let event_stream = stream.map(|chunk| match chunk {
        Ok(text) => Ok(Event::default().data(text)),
        Err(e) => Ok(Event::default().data(format!("[ERROR: {e}]"))),
    });

    Ok(Sse::new(event_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    ))
}
