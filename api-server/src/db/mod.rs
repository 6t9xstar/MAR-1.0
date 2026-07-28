pub mod migrations;

use eyre::Result;
use fred::clients::Pool as RedisPool;
use fred::interfaces::ClientLike;
use fred::types::config::Config as RedisConfig;
use meilisearch_sdk::client::Client as MeiliClient;
use qdrant_client::Qdrant;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use sqlx::ConnectOptions;

pub type Pool = PgPool;
pub type CachePool = RedisPool;
pub type VectorStore = Qdrant;
pub type SearchClient = MeiliClient;

pub async fn init_pool(config: &crate::config::DatabaseConfig) -> Result<Pool> {
    let mut opts: PgConnectOptions = config.url.parse()?;
    opts = opts
        .log_statements(tracing::log::LevelFilter::Debug)
        .log_slow_statements(tracing::log::LevelFilter::Warn, std::time::Duration::from_secs(1));

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(std::time::Duration::from_secs(config.acquire_timeout_secs))
        .idle_timeout(std::time::Duration::from_secs(config.idle_timeout_secs))
        .max_lifetime(std::time::Duration::from_secs(config.max_lifetime_secs))
        .connect_with(opts)
        .await?;

    sqlx::query("CREATE EXTENSION IF NOT EXISTS vector")
        .execute(&pool)
        .await?;

    tracing::info!("Database pool initialized: max={}", config.max_connections);
    Ok(pool)
}

pub async fn init_cache(config: &crate::config::CacheConfig) -> Result<CachePool> {
    let pool = RedisPool::new(RedisConfig::from_url(&config.redis_url)?, None, None, None, config.pool_size as usize)?;
    pool.connect();
    pool.wait_for_connect().await?;
    tracing::info!("Cache pool initialized: {} connections", config.pool_size);
    Ok(pool)
}

pub async fn init_vector_store(config: &crate::config::VectorDbConfig) -> Result<VectorStore> {
    let client = if config.api_key.is_empty() {
        Qdrant::from_url(&config.url).build()?
    } else {
        Qdrant::from_url(&config.url)
            .api_key(config.api_key.as_str())
            .build()?
    };

    let collections = client.list_collections().await?;
    let exists = collections.collections.iter()
        .any(|c| c.name == config.collection_name);

    if !exists {
        use qdrant_client::qdrant::{
            vectors_config, CreateCollection, Distance, HnswConfigDiff,
            OptimizersConfigDiff, VectorParams, VectorsConfig, WalConfigDiff,
        };

        client.create_collection(CreateCollection {
            collection_name: config.collection_name.clone(),
            vectors_config: Some(VectorsConfig {
                config: Some(vectors_config::Config::Params(VectorParams {
                    size: config.vector_size,
                    distance: Distance::Cosine.into(),
                    hnsw_config: Some(HnswConfigDiff {
                        m: Some(16),
                        ef_construct: Some(128),
                        full_scan_threshold: Some(10000),
                        ..Default::default()
                    }),
                    ..Default::default()
                })),
            }),
            optimizers_config: Some(OptimizersConfigDiff {
                indexing_threshold: Some(10000),
                ..Default::default()
            }),
            wal_config: Some(WalConfigDiff {
                wal_capacity_mb: Some(512),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await?;

        tracing::info!("Created vector collection: {}", config.collection_name);
    }

    tracing::info!("Vector store connected: {}", config.url);
    Ok(client)
}

pub async fn init_search(config: &crate::config::SearchConfig) -> Result<SearchClient> {
    let client = MeiliClient::new(&config.url, Some(&config.api_key))?;
    tracing::info!("Search client connected: {}", config.url);
    Ok(client)
}

pub async fn check_health(pool: &Pool, cache: &CachePool) -> Result<()> {
    sqlx::query("SELECT 1").execute(pool).await?;
    cache.ping::<()>(None).await?;
    Ok(())
}
