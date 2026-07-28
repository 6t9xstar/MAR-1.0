use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct Document {
    pub id: Uuid,
    pub user_id: Uuid,
    pub conversation_id: Option<Uuid>,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
    pub content_text: Option<String>,
    pub page_count: Option<u32>,
    pub status: DocumentStatus,
    pub embedding_status: EmbeddingStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DocumentStatus {
    Uploading,
    Processing,
    Ready,
    Error,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct DocumentSummary {
    pub id: Uuid,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
    pub page_count: Option<u32>,
    pub status: DocumentStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct UploadResponse {
    pub document: DocumentSummary,
    pub upload_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct DocumentQueryRequest {
    pub document_id: Uuid,
    pub query: String,
    pub max_results: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct DocumentQueryResult {
    pub chunk: String,
    pub page_number: Option<u32>,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ProcessedDocument {
    pub chunks: Vec<DocumentChunk>,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct DocumentChunk {
    pub index: u32,
    pub content: String,
    pub page_number: Option<u32>,
    pub embedding: Option<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct DocumentMetadata {
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
    pub page_count: Option<u32>,
    pub author: Option<String>,
    pub created_date: Option<DateTime<Utc>>,
    pub language: Option<String>,
}
