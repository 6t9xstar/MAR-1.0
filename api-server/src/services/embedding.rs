use crate::config::InferenceConfig;
use crate::errors::{AppError, AppResult};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct EmbeddingService {
    client: HttpClient,
    config: InferenceConfig,
    semaphore: Arc<Semaphore>,
}

#[derive(Serialize)]
struct EmbeddingRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
    model: String,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
    index: u32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Usage {
    prompt_tokens: u32,
    total_tokens: u32,
}

impl EmbeddingService {
    pub fn new(config: InferenceConfig) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        if !config.api_key.is_empty() {
            headers.insert(
                "Authorization",
                format!("Bearer {}", config.api_key).parse().unwrap(),
            );
        }

        let client = HttpClient::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create embedding HTTP client");

        let max_concurrent = config.max_concurrent_requests;
        Self {
            client,
            config,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    pub async fn embed(&self, texts: Vec<String>) -> AppResult<Vec<Vec<f32>>> {
        let _permit = self.semaphore.acquire().await.map_err(|_| {
            AppError::ServiceUnavailable("Too many embedding requests".into())
        })?;

        let request = EmbeddingRequest {
            model: self.config.model.clone(),
            input: texts,
        };

        let response = self
            .client
            .post(format!("{}/embeddings", self.config.api_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::Inference(format!("Embedding request failed: {e}")))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::Inference(format!("Embedding API returned {status}: {body}")));
        }

        let result: EmbeddingResponse = response
            .json()
            .await
            .map_err(|e| AppError::Inference(format!("Embedding parse failed: {e}")))?;

        let mut embeddings = vec![vec![0.0f32; result.data.first().map(|d| d.embedding.len()).unwrap_or(0)]; result.data.len()];
        for d in result.data {
            if let Some(slot) = embeddings.get_mut(d.index as usize) {
                *slot = d.embedding;
            }
        }

        if let Some(usage) = result.usage {
            tracing::info!(
                tokens = usage.total_tokens,
                embeddings = embeddings.len(),
                "Embeddings generated"
            );
        }

        Ok(embeddings)
    }
}
