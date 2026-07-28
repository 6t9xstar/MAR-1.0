use crate::config::InferenceConfig;
use crate::errors::{AppError, AppResult};
use crate::models::chat::{ChatMessage, MessageRole};
use crate::telemetry;
use futures::stream::Stream;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tracing::{error, info};

#[derive(Clone)]
pub struct InferenceClient {
    client: HttpClient,
    config: InferenceConfig,
    semaphore: Arc<Semaphore>,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<InferenceMessage>,
    max_tokens: u32,
    temperature: f32,
    stream: bool,
    stop: Option<Vec<String>>,
    frequency_penalty: Option<f32>,
    presence_penalty: Option<f32>,
}

#[derive(Debug, Serialize)]
struct InferenceMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ChatCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Choice {
    index: u32,
    message: Message,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Message {
    role: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

impl InferenceClient {
    pub fn new(config: InferenceConfig) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/json".parse().unwrap(),
        );
        if !config.api_key.is_empty() {
            headers.insert(
                "Authorization",
                format!("Bearer {}", config.api_key).parse().unwrap(),
            );
        }

        let client = HttpClient::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(config.request_timeout_secs))
            .pool_max_idle_per_host(config.max_concurrent_requests)
            .tcp_keepalive(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        let max_concurrent = config.max_concurrent_requests;
        Self {
            client,
            config,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        system_prompt: Option<&str>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> AppResult<String> {
        let _permit = self.semaphore.acquire().await.map_err(|_| {
            AppError::ServiceUnavailable("Too many concurrent inference requests".into())
        })?;

        let start = Instant::now();

        let mut inference_messages = Vec::new();

        if let Some(sys) = system_prompt {
            inference_messages.push(InferenceMessage {
                role: "system".into(),
                content: sys.to_string(),
            });
        }

        for msg in messages {
            inference_messages.push(InferenceMessage {
                role: match msg.role {
                    MessageRole::User => "user".into(),
                    MessageRole::Assistant => "assistant".into(),
                    MessageRole::System => "system".into(),
                },
                content: msg.content,
            });
        }

        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages: inference_messages,
            max_tokens: max_tokens.unwrap_or(self.config.max_tokens),
            temperature: temperature.unwrap_or(self.config.default_temperature),
            stream: false,
            stop: None,
            frequency_penalty: Some(0.0),
            presence_penalty: Some(0.0),
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.config.api_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Inference(format!("Request failed: {e}")))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Inference API error");
            return Err(AppError::Inference(format!("API returned {status}: {body}")));
        }

        let completion: ChatCompletionResponse = response
            .json()
            .await
            .map_err(|e| AppError::Inference(format!("Parse failed: {e}")))?;

        let content = completion
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .unwrap_or_default();

        let duration_ms = start.elapsed().as_secs_f64() * 1000.0;
        if let Some(usage) = completion.usage {
            telemetry::record_inference_tokens(&self.config.model, usage.total_tokens, duration_ms);
            info!(
                tokens = usage.total_tokens,
                prompt = usage.prompt_tokens,
                completion = usage.completion_tokens,
                duration_ms = duration_ms as u64,
                "Inference complete"
            );
        }

        Ok(content)
    }

pub async fn chat_stream(
    &self,
    messages: Vec<ChatMessage>,
    system_prompt: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
) -> AppResult<impl Stream<Item = Result<String, AppError>> + 'static + use<>> {
        let _permit = self.semaphore.acquire().await.map_err(|_| {
            AppError::ServiceUnavailable("Too many concurrent inference requests".into())
        })?;

        let mut inference_messages = Vec::new();

        if let Some(sys) = system_prompt {
            inference_messages.push(InferenceMessage {
                role: "system".into(),
                content: sys.to_string(),
            });
        }

        for msg in messages {
            inference_messages.push(InferenceMessage {
                role: match msg.role {
                    MessageRole::User => "user".into(),
                    MessageRole::Assistant => "assistant".into(),
                    MessageRole::System => "system".into(),
                },
                content: msg.content,
            });
        }

        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages: inference_messages,
            max_tokens: max_tokens.unwrap_or(self.config.max_tokens),
            temperature: temperature.unwrap_or(self.config.default_temperature),
            stream: true,
            stop: None,
            frequency_penalty: Some(0.0),
            presence_penalty: Some(0.0),
        };

        let client = self.client.clone();
        let api_url = self.config.api_url.clone();

        let stream = async_stream::stream! {
            let start = Instant::now();
            let mut total_tokens = 0u32;

            match client
                .post(format!("{api_url}/chat/completions"))
                .json(&request)
                .send()
                .await
            {
                Ok(response) => {
                    let status = response.status();
                    if !status.is_success() {
                        let body = response.text().await.unwrap_or_default();
                        error!(status = %status, body = %body, "Stream inference error");
                        yield Err(AppError::Inference(format!("API returned {status}: {body}")));
                        return;
                    }

                    let mut stream_data = response.bytes_stream();

                    use futures::StreamExt;
                    while let Some(chunk) = stream_data.next().await {
                        match chunk {
                            Ok(bytes) => {
                                let text = String::from_utf8_lossy(&bytes);
                                for line in text.lines() {
                                    let line = line.trim();
                                    if line.is_empty() || line == "data: [DONE]" {
                                        continue;
                                    }
                                    if let Some(data) = line.strip_prefix("data: ")
                                        && let Ok(chunk) = serde_json::from_str::<Value>(data)
                                        && let Some(choices) = chunk["choices"].as_array()
                                    {
                                        for choice in choices {
                                            if let Some(delta) = choice["delta"].as_object()
                                                && let Some(content) = delta.get("content").and_then(|c| c.as_str())
                                            {
                                                total_tokens += 1;
                                                yield Ok(content.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                error!(error = %e, "Stream read error");
                                yield Err(AppError::Inference(format!("Stream error: {e}")));
                                return;
                            }
                        }
                    }

                    let duration_ms = start.elapsed().as_secs_f64() * 1000.0;
                    info!(
                        tokens = total_tokens,
                        duration_ms = duration_ms as u64,
                        "Stream complete"
                    );
                }
                Err(e) => {
                    error!(error = %e, "Stream request failed");
                    yield Err(AppError::Inference(format!("Stream request failed: {e}")));
                }
            }
        };

        Ok(stream)
    }
}
