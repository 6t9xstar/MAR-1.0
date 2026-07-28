use sqlx::PgPool;
use tracing::info;

pub async fn run(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Running database migrations");

    sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"").execute(pool).await?;
    sqlx::query("CREATE EXTENSION IF NOT EXISTS vector").execute(pool).await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY,
            username VARCHAR(50) UNIQUE NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            display_name VARCHAR(100),
            preferred_language VARCHAR(10) DEFAULT 'en',
            is_verified BOOLEAN DEFAULT false,
            memory_enabled BOOLEAN DEFAULT true,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#
    ).execute(pool).await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS conversations (
            id UUID PRIMARY KEY,
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            title VARCHAR(200) NOT NULL DEFAULT 'New conversation',
            model VARCHAR(50) DEFAULT 'mar-7b',
            language VARCHAR(10) DEFAULT 'en',
            message_count INTEGER DEFAULT 0,
            token_count BIGINT DEFAULT 0,
            is_archived BOOLEAN DEFAULT false,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#
    ).execute(pool).await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS messages (
            id UUID PRIMARY KEY,
            conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
            role VARCHAR(20) NOT NULL,
            content TEXT NOT NULL,
            content_type VARCHAR(20) DEFAULT 'text',
            metadata JSONB,
            token_count INTEGER,
            latency_ms BIGINT,
            sources JSONB,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#
    ).execute(pool).await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS memories (
            id UUID PRIMARY KEY,
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            content TEXT NOT NULL,
            category VARCHAR(50) DEFAULT 'fact',
            importance SMALLINT DEFAULT 5,
            metadata JSONB,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            accessed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            expires_at TIMESTAMPTZ
        )"#
    ).execute(pool).await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS documents (
            id UUID PRIMARY KEY,
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            conversation_id UUID REFERENCES conversations(id) ON DELETE SET NULL,
            filename TEXT NOT NULL,
            original_filename TEXT NOT NULL,
            mime_type TEXT NOT NULL,
            size_bytes BIGINT NOT NULL,
            content_text TEXT,
            page_count INTEGER,
            status VARCHAR(20) DEFAULT 'uploading',
            embedding_status VARCHAR(20) DEFAULT 'pending',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#
    ).execute(pool).await?;

    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_conversations_user_id ON conversations(user_id)"#
    ).execute(pool).await?;
    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_conversations_updated ON conversations(updated_at DESC)"#
    ).execute(pool).await?;
    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id)"#
    ).execute(pool).await?;
    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_memories_user ON memories(user_id)"#
    ).execute(pool).await?;
    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_memories_category ON memories(category)"#
    ).execute(pool).await?;
    sqlx::query(
        r#"CREATE INDEX IF NOT EXISTS idx_documents_user ON documents(user_id)"#
    ).execute(pool).await?;

    info!("Database migrations complete");
    Ok(())
}
