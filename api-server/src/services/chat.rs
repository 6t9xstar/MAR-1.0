use crate::errors::{AppError, AppResult};
use crate::models::chat::{
    ChatMessage, ContentType, Conversation, ConversationSummary, MessageRole,
    SendMessageRequest, SourceCitation,
};
use crate::SharedState;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

pub struct ChatService;

impl ChatService {
    pub async fn create_conversation(
        db: &PgPool,
        user_id: Uuid,
        title: Option<String>,
        language: Option<String>,
    ) -> AppResult<Conversation> {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let title = title.unwrap_or_else(|| "New conversation".into());
        let language = language.unwrap_or_else(|| "en".into());

        sqlx::query(
            r#"INSERT INTO conversations (id, user_id, title, language, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6)"#
        )
        .bind(id)
        .bind(user_id)
        .bind(&title)
        .bind(&language)
        .bind(now)
        .bind(now)
        .execute(db)
        .await?;

        info!("Conversation created: {id}");
        Ok(Conversation {
            id,
            user_id,
            title,
            model: "mar-8b".into(),
            language,
            message_count: 0,
            token_count: 0,
            is_archived: false,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_conversations(
        db: &PgPool,
        user_id: Uuid,
        page: u64,
        per_page: u64,
    ) -> AppResult<(Vec<ConversationSummary>, u64)> {
        let offset = (page.saturating_sub(1)) * per_page;
        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM conversations WHERE user_id = $1 AND is_archived = false"
        )
        .bind(user_id)
        .fetch_one(db)
        .await?;

        let rows = sqlx::query_as::<_, (Uuid, String, i32, String, bool, DateTime<Utc>, DateTime<Utc>)>(
            r#"SELECT id, title, message_count, language, is_archived, created_at, updated_at
               FROM conversations
               WHERE user_id = $1 AND is_archived = false
               ORDER BY updated_at DESC
               LIMIT $2 OFFSET $3"#
        )
        .bind(user_id)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(db)
        .await?;

        let summaries = rows
            .into_iter()
            .map(|r| ConversationSummary {
                id: r.0,
                title: r.1,
                last_message_preview: None,
                message_count: r.2 as u32,
                language: r.3,
                is_archived: r.4,
                created_at: r.5,
                updated_at: r.6,
            })
            .collect();

        Ok((summaries, total.0 as u64))
    }

    pub async fn get_conversation(db: &PgPool, user_id: Uuid, conv_id: Uuid) -> AppResult<Conversation> {
        sqlx::query_as::<_, (Uuid, Uuid, String, String, String, i32, i64, bool, DateTime<Utc>, DateTime<Utc>)>(
            r#"SELECT id, user_id, title, model, language, message_count,
                      token_count, is_archived, created_at, updated_at
               FROM conversations WHERE id = $1 AND user_id = $2"#
        )
        .bind(conv_id)
        .bind(user_id)
        .fetch_optional(db)
        .await?
        .map(|r| Conversation {
            id: r.0, user_id: r.1, title: r.2, model: r.3,
            language: r.4, message_count: r.5 as u32, token_count: r.6 as u64,
            is_archived: r.7, created_at: r.8, updated_at: r.9,
        })
        .ok_or_else(|| AppError::NotFound("Conversation not found".into()))
    }

    pub async fn get_messages(
        db: &PgPool,
        user_id: Uuid,
        conv_id: Uuid,
        page: u64,
        per_page: u64,
    ) -> AppResult<Vec<ChatMessage>> {
        let offset = (page.saturating_sub(1)) * per_page;

        let rows = sqlx::query_as::<_, (Uuid, String, String, String, Option<serde_json::Value>, Option<i32>, Option<i64>, Option<serde_json::Value>, DateTime<Utc>)>(
            r#"SELECT id, role, content, content_type, metadata, token_count, latency_ms, sources, created_at
               FROM messages
               WHERE conversation_id = $1 AND conversation_id IN (
                   SELECT id FROM conversations WHERE user_id = $2
               )
               ORDER BY created_at ASC LIMIT $3 OFFSET $4"#
        )
        .bind(conv_id)
        .bind(user_id)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(db)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| ChatMessage {
                id: r.0,
                conversation_id: conv_id,
                role: match r.1.as_str() {
                    "user" => MessageRole::User,
                    "assistant" => MessageRole::Assistant,
                    _ => MessageRole::System,
                },
                content: r.2,
                content_type: match r.3.as_str() {
                    "code" => ContentType::Code,
                    "image" => ContentType::Image,
                    "document" => ContentType::Document,
                    "audio" => ContentType::Audio,
                    "error" => ContentType::Error,
                    _ => ContentType::Text,
                },
                metadata: r.4,
                token_count: r.5.map(|t| t as u32),
                latency_ms: r.6.map(|l| l as u64),
                sources: r.7
                    .and_then(|s| serde_json::from_value::<Vec<SourceCitation>>(s).ok())
                    .unwrap_or_default(),
                created_at: r.8,
            })
            .collect())
    }

    pub async fn send_message(
        state: &SharedState,
        user_id: Uuid,
        request: SendMessageRequest,
    ) -> AppResult<(ChatMessage, Uuid)> {
        let conv_id = match request.conversation_id {
            Some(id) => {
                Self::get_conversation(&state.db, user_id, id).await?;
                id
            }
            None => {
                let conv = Self::create_conversation(
                    &state.db,
                    user_id,
                    None,
                    request.language.clone(),
                )
                .await?;
                conv.id
            }
        };

        let user_msg_id = Uuid::now_v7();
        let now = Utc::now();
        let content_type = request.content_type.unwrap_or(ContentType::Text);

        sqlx::query(
            r#"INSERT INTO messages (id, conversation_id, role, content, content_type, created_at)
               VALUES ($1, $2, $3, $4, $5, $6)"#
        )
        .bind(user_msg_id)
        .bind(conv_id)
        .bind("user")
        .bind(&request.content)
        .bind(format!("{content_type:?}").to_lowercase())
        .bind(now)
        .execute(&state.db)
        .await?;

        sqlx::query(
            "UPDATE conversations SET message_count = message_count + 1, updated_at = $1 WHERE id = $2"
        )
        .bind(now)
        .bind(conv_id)
        .execute(&state.db)
        .await?;

        let _conversation = Self::get_conversation(&state.db, user_id, conv_id).await?;
        let messages = Self::get_messages(&state.db, user_id, conv_id, 1, 1000).await?;

        let language = state.nlp.detect_language(&request.content);
        let domain = state.knowledge.domain_for_text(&request.content, &format!("{:?}", language));
        let system_prompt = state.prompt_builder.build(
            &language,
            &domain,
            Some(&state.knowledge),
            None,
            &request.content,
        );

        let response_content = if request.stream.unwrap_or(false) {
            return Err(AppError::BadRequest("Use stream endpoint for streaming".into()));
        } else {
            state.inference.chat(
                messages,
                Some(&system_prompt),
                request.temperature,
                request.max_tokens,
            ).await?
        };

        let relevant_chunks = state.knowledge.relevant_knowledge(
            &request.content,
            &format!("{:?}", language),
            3,
        );

        let citations: Vec<SourceCitation> = relevant_chunks.iter()
            .flat_map(|chunk| chunk.citations.iter())
            .map(|c| SourceCitation {
                title: c.source.clone(),
                url: c.url.clone(),
                snippet: c.text.clone(),
                relevance_score: 0.9,
            })
            .collect();

        let final_content = if let Some(domain) = state.knowledge.requires_disclaimer(
            &request.content,
            &format!("{:?}", language),
        ) {
            let disclaimer = state.safety.disclaimer_for(&domain);
            match disclaimer {
                Some(d) => format!("{}\n\n{}", d, response_content),
                None => response_content,
            }
        } else {
            response_content
        };

        let assistant_msg_id = Uuid::now_v7();
        sqlx::query(
            r#"INSERT INTO messages (id, conversation_id, role, content, content_type, metadata, sources, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#
        )
        .bind(assistant_msg_id)
        .bind(conv_id)
        .bind("assistant")
        .bind(&final_content)
        .bind("text")
        .bind(serde_json::json!({
            "language": format!("{:?}", language),
            "domain": format!("{:?}", domain),
            "citations_count": citations.len(),
        }))
        .bind(serde_json::to_value(&citations).ok())
        .bind(Utc::now())
        .execute(&state.db)
        .await?;

        sqlx::query(
            "UPDATE conversations SET message_count = message_count + 1, updated_at = $1 WHERE id = $2"
        )
        .bind(Utc::now())
        .bind(conv_id)
        .execute(&state.db)
        .await?;

        Ok((
            ChatMessage {
                id: assistant_msg_id,
                conversation_id: conv_id,
                role: MessageRole::Assistant,
                content: final_content,
                content_type: ContentType::Text,
                metadata: Some(serde_json::json!({
                    "language": format!("{:?}", language),
                    "domain": format!("{:?}", domain),
                })),
                token_count: None,
                latency_ms: None,
                sources: citations,
                created_at: Utc::now(),
            },
            conv_id,
        ))
    }

    pub fn build_system_prompt(language: &str, memory_enabled: Option<bool>) -> String {
        let lang_instruction = match language {
            "ur" => "You are MAR, an AI assistant for Pakistan. Respond in Urdu (اردو). Be helpful, accurate, and concise.",
            "roman-urdu" => "You are MAR, an AI assistant for Pakistan. Respond in Roman Urdu (Urdu written in English script). Be helpful, accurate, and concise.",
            "pnb" => "You are MAR, an AI assistant for Pakistan. Respond in Punjabi (پنجابی). Be helpful, accurate, and concise.",
            _ => "You are MAR, an AI assistant built for Pakistan. Respond in English. Be helpful, accurate, and concise.",
        };

        let memory_instruction = if memory_enabled.unwrap_or(true) {
            "You have access to the user's memory. Use it to provide personalized, context-aware responses."
        } else {
            ""
        };

        format!(
            "{}\n{}\n\nYou have excellent knowledge of Pakistan: its culture, laws, education system, business environment, agriculture, and Islamic studies. Cite sources when possible. If you're unsure, admit it. Be warm and conversational.",
            lang_instruction, memory_instruction
        )
    }
}
