#![allow(clippy::needless_borrow)]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_health_endpoint() {
    let config = api_server::config::Config::load().expect("Failed to load config");
    
    let pool = api_server::db::init_pool(&config.database)
        .await
        .expect("Failed to init pool");
    
    let cache = api_server::db::init_cache(&config.cache)
        .await
        .expect("Failed to init cache");
    
    let vector_store = api_server::db::init_vector_store(&config.vector_db)
        .await
        .expect("Failed to init vector store");
    
    let search = api_server::db::init_search(&config.search)
        .await
        .expect("Failed to init search");
    
    let inference = api_server::services::inference::InferenceClient::new(config.inference.clone());
    let memory = api_server::services::memory::MemoryService::new(
        pool.clone(),
        Arc::new(vector_store),
        &config,
    );
    
    let state = Arc::new(api_server::AppState {
        config: config.clone(),
        db: pool,
        cache,
        vector_store: Arc::new(qdrant_client::Qdrant::from_url("http://localhost:6333").build().unwrap()),
        search,
        inference,
        memory,
        nlp: api_server::services::nlp::NlpService::new(&config.nlp),
        knowledge: api_server::knowledge::KnowledgeService::new("./data/knowledge"),
        prompt_builder: api_server::services::prompts::PromptBuilder::new(),
        safety: api_server::services::safety::SafetyService::new(),
    });
    
    let app = api_server::build_app(state).await;
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_version_endpoint() {
    let config = api_server::config::Config::load().expect("Failed to load config");
    let pool = api_server::db::init_pool(&config.database)
        .await
        .expect("Failed to init pool");
    let cache = api_server::db::init_cache(&config.cache)
        .await
        .expect("Failed to init cache");
    
    let state = Arc::new(api_server::AppState {
        config: config.clone(),
        db: pool,
        cache,
        vector_store: Arc::new(qdrant_client::Qdrant::from_url("http://localhost:6333").build().unwrap()),
        search: meilisearch_sdk::client::Client::new("http://localhost:7700", Some("mar-search-key")).unwrap(),
        inference: api_server::services::inference::InferenceClient::new(config.inference.clone()),
        memory: api_server::services::memory::MemoryService::new(
            sqlx::PgPool::connect("postgres://mar:mar@localhost:5432/mar").await.unwrap(),
            Arc::new(qdrant_client::Qdrant::from_url("http://localhost:6333").build().unwrap()),
            config,
        ),
        nlp: api_server::services::nlp::NlpService::new(&config.nlp),
        knowledge: api_server::knowledge::KnowledgeService::new("./data/knowledge"),
        prompt_builder: api_server::services::prompts::PromptBuilder::new(),
        safety: api_server::services::safety::SafetyService::new(),
    });
    
    let app = api_server::build_app(state).await;
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/version")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
