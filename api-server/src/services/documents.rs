use crate::config::UploadConfig;
use crate::errors::{AppError, AppResult};
use crate::models::document::{
    Document, DocumentQueryResult, DocumentStatus,
    DocumentSummary, EmbeddingStatus,
};
use calamine::Reader;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::path::PathBuf;
use tokio::fs;
use tracing::info;
use uuid::Uuid;

pub struct DocumentService;

impl DocumentService {
    pub async fn upload(
        db: &PgPool,
        config: &UploadConfig,
        user_id: Uuid,
        conversation_id: Option<Uuid>,
        filename: &str,
        data: &[u8],
    ) -> AppResult<DocumentSummary> {
        if data.len() as u64 > config.max_file_size {
            return Err(AppError::FileTooLarge(config.max_file_size));
        }

        let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
        if !config.allowed_extensions.contains(&ext) {
            return Err(AppError::UnsupportedFileType(ext));
        }

        let id = Uuid::now_v7();
        let storage_name = format!("{id}.{ext}");
        let storage_path = config.storage_path.join(&storage_name);

        fs::create_dir_all(&config.storage_path).await?;
        fs::write(&storage_path, data).await?;

        let mime_type = mime_guess::from_path(filename)
            .first_or_octet_stream()
            .to_string();

        let content_text = Self::extract_text(&storage_path, &ext).await;

        sqlx::query(
            r#"INSERT INTO documents (id, user_id, conversation_id, filename, original_filename,
               mime_type, size_bytes, content_text, status, created_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#
        )
        .bind(id)
        .bind(user_id)
        .bind(conversation_id)
        .bind(&storage_name)
        .bind(filename)
        .bind(&mime_type)
        .bind(data.len() as i64)
        .bind(&content_text)
        .bind("ready")
        .bind(Utc::now())
        .execute(db)
        .await?;

        info!("Document uploaded: {id} ({filename})");
        self::get_summary(db, id, user_id).await
    }

    pub async fn get_document(
        db: &PgPool,
        doc_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Document> {
        sqlx::query_as::<_, (Uuid, Uuid, Option<Uuid>, String, String, String, i64, Option<String>, Option<i32>, String, String, DateTime<Utc>)>(
            r#"SELECT id, user_id, conversation_id, filename, original_filename,
                      mime_type, size_bytes, content_text, page_count, status, embedding_status, created_at
               FROM documents WHERE id = $1 AND user_id = $2"#
        )
        .bind(doc_id)
        .bind(user_id)
        .fetch_optional(db)
        .await?
        .map(|r| Document {
            id: r.0, user_id: r.1, conversation_id: r.2,
            filename: r.3, original_filename: r.4,
            mime_type: r.5, size_bytes: r.6 as u64,
            content_text: r.7, page_count: r.8.map(|p| p as u32),
            status: match r.9.as_str() {
                "uploading" => DocumentStatus::Uploading,
                "processing" => DocumentStatus::Processing,
                "ready" => DocumentStatus::Ready,
                "error" => DocumentStatus::Error,
                _ => DocumentStatus::Deleted,
            },
            embedding_status: match r.10.as_str() {
                "processing" => EmbeddingStatus::Processing,
                "completed" => EmbeddingStatus::Completed,
                "failed" => EmbeddingStatus::Failed,
                _ => EmbeddingStatus::Pending,
            },
            created_at: r.11,
        })
        .ok_or_else(|| AppError::NotFound("Document not found".into()))
    }

    pub async fn list_documents(
        db: &PgPool,
        user_id: Uuid,
        page: u64,
        per_page: u64,
    ) -> AppResult<(Vec<DocumentSummary>, u64)> {
        let offset = (page.saturating_sub(1)) * per_page;
        let total: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM documents WHERE user_id = $1 AND status != 'deleted'"
        )
        .bind(user_id)
        .fetch_one(db)
        .await?;

        let rows = sqlx::query_as::<_, (Uuid, String, String, String, i64, Option<i32>, String, DateTime<Utc>)>(
            r#"SELECT id, filename, original_filename, mime_type, size_bytes, page_count, status, created_at
               FROM documents WHERE user_id = $1 AND status != 'deleted'
               ORDER BY created_at DESC LIMIT $2 OFFSET $3"#
        )
        .bind(user_id)
        .bind(per_page as i64)
        .bind(offset as i64)
        .fetch_all(db)
        .await?;

        let summaries = rows
            .into_iter()
            .map(|r| DocumentSummary {
                id: r.0, filename: r.1, original_filename: r.2,
                mime_type: r.3, size_bytes: r.4 as u64,
                page_count: r.5.map(|p| p as u32),
                status: match r.6.as_str() {
                    "uploading" => DocumentStatus::Uploading,
                    "processing" => DocumentStatus::Processing,
                    "ready" => DocumentStatus::Ready,
                    "error" => DocumentStatus::Error,
                    _ => DocumentStatus::Deleted,
                },
                created_at: r.7,
            })
            .collect();

        Ok((summaries, total.0 as u64))
    }

    pub async fn query_document(
        db: &PgPool,
        doc_id: Uuid,
        user_id: Uuid,
        query: &str,
        max_results: u32,
    ) -> AppResult<Vec<DocumentQueryResult>> {
        let doc = Self::get_document(db, doc_id, user_id).await?;
        let content = doc.content_text.unwrap_or_default();
        let max_results = max_results.clamp(1, 20);

        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for (i, chunk) in content.split('\n').enumerate() {
            if results.len() >= max_results as usize {
                break;
            }
            if chunk.to_lowercase().contains(&query_lower) {
                results.push(DocumentQueryResult {
                    chunk: chunk.to_string(),
                    page_number: None,
                    relevance_score: 1.0 - (i as f64 / content.len() as f64),
                });
            }
        }

        if results.is_empty() {
            let chunk_size = (content.len() / max_results as usize).max(100);
            for i in 0..max_results as usize {
                let start = (i * chunk_size).min(content.len());
                let end = ((i + 1) * chunk_size).min(content.len());
                if start < end {
                    results.push(DocumentQueryResult {
                        chunk: content[start..end].to_string(),
                        page_number: None,
                        relevance_score: 0.1,
                    });
                }
            }
        }

        Ok(results)
    }

    pub async fn delete_document(db: &PgPool, doc_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let doc = Self::get_document(db, doc_id, user_id).await?;
        let path = config.upload.storage_path.clone().join(&doc.filename);
        let _ = fs::remove_file(path).await;

        sqlx::query("UPDATE documents SET status = 'deleted' WHERE id = $1 AND user_id = $2")
            .bind(doc_id)
            .bind(user_id)
            .execute(db)
            .await?;

        info!("Document deleted: {doc_id}");
        Ok(())
    }

    fn extract_docx_text(docx: &docx_rs::Docx) -> String {
        let mut text = String::new();
        for child in &docx.document.children {
            if let docx_rs::DocumentChild::Paragraph(p) = child {
                for run in &p.children {
                    if let docx_rs::ParagraphChild::Run(r) = run {
                        for c in &r.children {
                            if let docx_rs::RunChild::Text(t) = c {
                                text.push_str(&t.text);
                            }
                        }
                    }
                }
                text.push('\n');
            }
        }
        text
    }

    async fn extract_text(path: &PathBuf, ext: &str) -> Option<String> {
        match ext {
            "txt" | "md" | "csv" => fs::read_to_string(path).await.ok(),
            "pdf" => pdf_extract::extract_text(path).ok(),
            "docx" => {
                let bytes = fs::read(path).await.ok()?;
                let docx = docx_rs::read_docx(&bytes).ok()?;
                Some(Self::extract_docx_text(&docx))
            }
            "xlsx" => {
                let bytes = fs::read(path).await.ok()?;
                let mut workbook: calamine::Xlsx<_> = calamine::open_workbook_from_rs(std::io::Cursor::new(bytes)).ok()?;
                let mut text = String::new();
                let sheet_names = workbook.sheet_names().to_vec();
                for name in sheet_names {
                    if let Ok(range) = workbook.worksheet_range(&name) {
                        for row in range.rows() {
                            for cell in row {
                                text.push_str(&cell.to_string());
                                text.push('\t');
                            }
                            text.push('\n');
                        }
                    }
                }
                Some(text)
            }
            _ => None,
        }
    }
}

async fn get_summary(db: &PgPool, doc_id: Uuid, user_id: Uuid) -> AppResult<DocumentSummary> {
    let doc = DocumentService::get_document(db, doc_id, user_id).await?;
    Ok(DocumentSummary {
        id: doc.id,
        filename: doc.filename,
        original_filename: doc.original_filename,
        mime_type: doc.mime_type,
        size_bytes: doc.size_bytes,
        page_count: doc.page_count,
        status: doc.status,
        created_at: doc.created_at,
    })
}

