use crate::config::Config;
use crate::errors::{AppError, AppResult};
use crate::models::memory::{
    CreateMemoryRequest, MemoryCategory, MemoryEntry, MemorySearchResult, MemoryStats,
    UpdateMemoryRequest,
};
use chrono::{DateTime, Utc};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    Condition, DeletePointsBuilder, Filter, PointStruct, SearchPointsBuilder,
    UpsertPointsBuilder,
};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct MemoryService {
    db: PgPool,
    vector_store: Arc<Qdrant>,
    collection_name: String,
    vector_size: u64,
}

impl MemoryService {
    pub fn new(db: PgPool, vector_store: Arc<Qdrant>, config: &Config) -> Self {
        Self {
            db,
            vector_store,
            collection_name: config.vector_db.collection_name.clone(),
            vector_size: config.vector_db.vector_size,
        }
    }

    pub async fn create_memory(
        &self,
        user_id: Uuid,
        request: CreateMemoryRequest,
    ) -> AppResult<MemoryEntry> {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let expires_at = request.ttl_seconds.map(|s| now + chrono::Duration::seconds(s as i64));

        let category = request.category.unwrap_or(MemoryCategory::Fact);
        let importance = request.importance.unwrap_or(5).min(10);

        sqlx::query(
            r#"INSERT INTO memories (id, user_id, content, category, importance, metadata, expires_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7)"#
        )
        .bind(id)
        .bind(user_id)
        .bind(&request.content)
        .bind(format!("{category:?}"))
        .bind(importance as i16)
        .bind(&request.metadata)
        .bind(expires_at)
        .execute(&self.db)
        .await?;

        // Generate embedding and store in Qdrant asynchronously
        let content_clone = request.content.clone();
        let vs = self.vector_store.clone();
        let coll = self.collection_name.clone();
        let vsize = self.vector_size;
        tokio::spawn(async move {
            // In production, call the embedding service here
            let mock_embedding: Vec<f32> = vec![0.0; vsize as usize];
            let mut payload = HashMap::new();
            payload.insert("user_id".to_string(), serde_json::Value::String(user_id.to_string()));
            payload.insert("content".to_string(), serde_json::Value::String(content_clone));
            payload.insert("created_at".to_string(), serde_json::Value::String(now.to_rfc3339()));
            let points = vec![PointStruct::new(
                id.to_string(),
                mock_embedding,
                payload,
            )];
            let _ = vs.upsert_points(
                UpsertPointsBuilder::new(coll, points)
            ).await;
        });

        info!("Memory created: {id}");
        Ok(MemoryEntry {
            id,
            user_id,
            content: request.content,
            category,
            importance,
            embedding: None,
            metadata: request.metadata,
            created_at: now,
            accessed_at: now,
            expires_at,
        })
    }

    pub async fn search_memories(
        &self,
        user_id: Uuid,
        query: &str,
        category: Option<MemoryCategory>,
        limit: u32,
        min_importance: u8,
    ) -> AppResult<Vec<MemorySearchResult>> {
        let limit = limit.min(50);

        // First try vector search
        let mut filter = Filter::must(vec![Condition::matches(
            "user_id",
            user_id.to_string(),
        )]);

        if let Some(cat) = category {
            filter.must.push(Condition::matches(
                "category",
                format!("{cat:?}"),
            ));
        }

        let search_result = self.vector_store
            .search_points(
                SearchPointsBuilder::new(
                    self.collection_name.clone(),
                    vec![0.0; self.vector_size as usize],
                    limit as u64,
                )
                .filter(filter)
                .with_payload(true)
            )
            .await
            .map_err(|e| AppError::Internal(format!("Vector search failed: {e}")))?;

        let mut results = Vec::new();
        for point in search_result.result {
            if let Some(content) = point.payload.get("content").and_then(|p| p.as_str()) {
                results.push(MemorySearchResult {
                    relevance_score: point.score as f64,
                    memory: MemoryEntry {
                        id: Uuid::parse_str(&format!("{:?}", point.id)).unwrap_or_default(),
                        user_id,
                        content: content.to_string(),
                        category: MemoryCategory::Fact,
                        importance: 5,
                        embedding: None,
                        metadata: None,
                        created_at: Utc::now(),
                        accessed_at: Utc::now(),
                        expires_at: None,
                    },
                });
            }
        }

        // Fall back to text search if vector results are insufficient
        if results.is_empty() {
            let rows = sqlx::query_as::<_, (Uuid, String, String, i16, Option<serde_json::Value>, DateTime<Utc>, DateTime<Utc>, Option<DateTime<Utc>>)>(
                r#"SELECT id, content, category, importance, metadata, created_at, accessed_at, expires_at
                   FROM memories
                   WHERE user_id = $1
                   AND content ILIKE $2
                   AND importance >= $3
                   ORDER BY created_at DESC
                   LIMIT $4"#
            )
            .bind(user_id)
            .bind(format!("%{}%", query))
            .bind(min_importance as i16)
            .bind(limit as i64)
            .fetch_all(&self.db)
            .await?;

            for (id, content, cat_str, importance, metadata, created_at, accessed_at, expires_at) in rows {
                let category = match cat_str.to_lowercase().as_str() {
                    "fact" => MemoryCategory::Fact,
                    "preference" => MemoryCategory::Preference,
                    "context" => MemoryCategory::Context,
                    "conversation" => MemoryCategory::Conversation,
                    "learned" => MemoryCategory::Learned,
                    _ => MemoryCategory::UserDefined(cat_str),
                };

                results.push(MemorySearchResult {
                    relevance_score: 0.5,
                    memory: MemoryEntry {
                        id, user_id, content, category,
                        importance: importance as u8,
                        embedding: None,
                        metadata,
                        created_at, accessed_at, expires_at,
                    },
                });
            }
        }

        Ok(results)
    }

    pub async fn get_memory(&self, user_id: Uuid, memory_id: Uuid) -> AppResult<MemoryEntry> {
        let row = sqlx::query_as::<_, (Uuid, String, String, i16, Option<serde_json::Value>, DateTime<Utc>, DateTime<Utc>, Option<DateTime<Utc>>)>(
            r#"SELECT id, content, category, importance, metadata, created_at, accessed_at, expires_at
               FROM memories WHERE id = $1 AND user_id = $2"#
        )
        .bind(memory_id)
        .bind(user_id)
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Memory not found".into()))?;

        Ok(MemoryEntry {
            id: row.0,
            user_id,
            content: row.1,
            category: MemoryCategory::Fact,
            importance: row.3 as u8,
            embedding: None,
            metadata: row.4,
            created_at: row.5,
            accessed_at: row.6,
            expires_at: row.7,
        })
    }

    pub async fn update_memory(
        &self,
        user_id: Uuid,
        memory_id: Uuid,
        request: UpdateMemoryRequest,
    ) -> AppResult<MemoryEntry> {
        let existing = self.get_memory(user_id, memory_id).await?;

        let content = request.content.unwrap_or(existing.content);
        let importance = request.importance.unwrap_or(existing.importance);
        let metadata = request.metadata.or(existing.metadata);

        sqlx::query(
            r#"UPDATE memories SET content = $1, importance = $2, metadata = $3
               WHERE id = $4 AND user_id = $5"#
        )
        .bind(&content)
        .bind(importance as i16)
        .bind(&metadata)
        .bind(memory_id)
        .bind(user_id)
        .execute(&self.db)
        .await?;

        self.get_memory(user_id, memory_id).await
    }

    pub async fn delete_memory(&self, user_id: Uuid, memory_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM memories WHERE id = $1 AND user_id = $2")
            .bind(memory_id)
            .bind(user_id)
            .execute(&self.db)
            .await?;

        let _ = self.vector_store
            .delete_points(
                DeletePointsBuilder::new(self.collection_name.clone())
                    .points([memory_id.to_string()])
            )
            .await;

        Ok(())
    }

    pub async fn list_memories(
        &self,
        user_id: Uuid,
        page: u64,
        per_page: u64,
    ) -> AppResult<(Vec<MemoryEntry>, u64)> {
        let offset = (page.saturating_sub(1)) * per_page;

        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM memories WHERE user_id =  AND (expires_at IS NULL OR expires_at > NOW())()"
        )
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

        let rows = sqlx::query_as::<_, (Uuid, String, String, i16, Option<serde_json::Value>, DateTime<Utc>, DateTime<Utc>, Option<DateTime<Utc>>)>(
            r#"SELECT id, content, category, importance, metadata, created_at, accessed_at, expires_at
               FROM memories WHERE user_id = $1
               ORDER BY created_at DESC LIMIT $2 OFFSET $3"#
        )
        .bind(user_id)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(&self.db)
        .await?;

        let entries = rows
            .into_iter()
            .map(|r| MemoryEntry {
                id: r.0,
                user_id,
                content: r.1,
                category: MemoryCategory::Fact,
                importance: r.3 as u8,
                embedding: None,
                metadata: r.4,
                created_at: r.5,
                accessed_at: r.6,
                expires_at: r.7,
            })
            .collect();

        Ok((entries, total.0 as u64))
    }

    pub async fn get_stats(&self, user_id: Uuid) -> AppResult<MemoryStats> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM memories WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db)
            .await?;

        let avg_imp: (Option<f64>,) = sqlx::query_as(
            "SELECT AVG(importance::float) FROM memories WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

        Ok(MemoryStats {
            total_count: total.0 as u64,
            category_counts: std::collections::HashMap::new(),
            oldest_memory: None,
            newest_memory: None,
            average_importance: avg_imp.0.unwrap_or(0.0),
        })
    }

    pub async fn clear_memories(&self, user_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM memories WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.db)
            .await?;

        let _ = self.vector_store
            .delete_points(
                DeletePointsBuilder::new(self.collection_name.clone())
                    .points(Filter::must(vec![Condition::matches(
                        "user_id",
                        user_id.to_string(),
                    )]))
            )
            .await;

        Ok(())
    }
}
