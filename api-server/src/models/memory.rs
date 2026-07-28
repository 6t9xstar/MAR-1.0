use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct MemoryEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub category: MemoryCategory,
    pub importance: u8,
    pub embedding: Option<Vec<f32>>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum MemoryCategory {
    Fact,
    Preference,
    Context,
    Conversation,
    Learned,
    UserDefined(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct CreateMemoryRequest {
    pub content: String,
    pub category: Option<MemoryCategory>,
    pub importance: Option<u8>,
    pub metadata: Option<serde_json::Value>,
    pub ttl_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct UpdateMemoryRequest {
    pub content: Option<String>,
    pub category: Option<MemoryCategory>,
    pub importance: Option<u8>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct MemoryQuery {
    pub query: String,
    pub category: Option<MemoryCategory>,
    pub limit: Option<u32>,
    pub min_importance: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct MemorySearchResult {
    pub memory: MemoryEntry,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct MemoryStats {
    pub total_count: u64,
    pub category_counts: std::collections::HashMap<String, u64>,
    pub oldest_memory: Option<DateTime<Utc>>,
    pub newest_memory: Option<DateTime<Utc>>,
    pub average_importance: f64,
}
