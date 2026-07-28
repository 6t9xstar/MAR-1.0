use axum::{
    extract::State,
    response::sse::{Event, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use crate::engine::InferenceEngine;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<InferenceEngine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: ResponseMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}

pub fn router(engine: Arc<InferenceEngine>) -> Router {
    let state = AppState { engine };

    Router::new()
        .route("/v1/chat/completions", post(chat_completions))
        .route("/v1/models", get(list_models))
        .route("/v1/health", get(health))
        .with_state(state)
}

async fn chat_completions(
    State(state): State<AppState>,
    Json(req): Json<ChatCompletionRequest>,
) -> axum::response::Response {
    let prompt = format_messages(&req.messages);

    if req.stream.unwrap_or(false) {
        match state.engine.generate_stream(&prompt, req.max_tokens, req.temperature).await {
            Ok(rx) => {
                let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
                let sse_stream = stream.map(|token| {
                    Ok::<_, Infallible>(Event::default().data(token))
                });

                Sse::new(sse_stream)
                    .keep_alive(
                        axum::response::sse::KeepAlive::new()
                            .interval(Duration::from_secs(15))
                            .text("keep-alive"),
                    )
                    .into_response()
            }
            Err(e) => {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": e.to_string()})),
                )
                    .into_response()
            }
        }
    } else {
        match state.engine.generate(&prompt, req.max_tokens, req.temperature).await {
            Ok(content) => {
                let response = ChatCompletionResponse {
                    id: uuid::Uuid::now_v7().to_string(),
                    object: "chat.completion".into(),
                    created: chrono::Utc::now().timestamp() as u64,
                    model: req.model,
                    choices: vec![Choice {
                        index: 0,
                        message: ResponseMessage {
                            role: "assistant".into(),
                            content,
                        },
                        finish_reason: Some("stop".into()),
                    }],
                    usage: Some(Usage {
                        prompt_tokens: 0,
                        completion_tokens: 0,
                        total_tokens: 0,
                    }),
                };

                (axum::http::StatusCode::OK, Json(response)).into_response()
            }
            Err(e) => {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": e.to_string()})),
                )
                    .into_response()
            }
        }
    }
}

fn format_messages(messages: &[Message]) -> String {
    let mut prompt = String::new();
    for msg in messages {
        match msg.role.as_str() {
            "system" => prompt.push_str(&format!("<|system|>\n{}\n", msg.content)),
            "user" => prompt.push_str(&format!("<|user|>\n{}\n", msg.content)),
            "assistant" => prompt.push_str(&format!("<|assistant|>\n{}\n", msg.content)),
            _ => prompt.push_str(&format!("{}\n", msg.content)),
        }
    }
    prompt.push_str("<|assistant|>\n");
    prompt
}

async fn list_models(State(state): State<AppState>) -> Json<Vec<ModelInfo>> {
    Json(vec![ModelInfo {
        id: state.engine.model_name(),
        object: "model".into(),
        created: 1700000000,
        owned_by: "MAR".into(),
    }])
}

async fn health(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "inference",
        "model": state.engine.model_name(),
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}
