use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use std::sync::Arc;
use tower::ServiceExt;

async fn setup_app() -> Router {
    let config = api_server::config::Config::load().expect("Failed to load config");
    let pool = api_server::db::init_pool(&config.database)
        .await
        .expect("Failed to init pool");
    let cache = api_server::db::init_cache(&config.cache)
        .await
        .expect("Failed to init cache");
    
    let state = Arc::new(api_server::AppState {
        config: config.clone(),
        db: pool.clone(),
        cache,
        vector_store: Arc::new(qdrant_client::Qdrant::from_url("http://localhost:6333").build().unwrap()),
        search: meilisearch_sdk::client::Client::new("http://localhost:7700", Some("mar-search-key")).unwrap(),
        inference: api_server::services::inference::InferenceClient::new(config.inference.clone()),
        memory: api_server::services::memory::MemoryService::new(
            pool,
            Arc::new(qdrant_client::Qdrant::from_url("http://localhost:6333").build().unwrap()),
            config,
        ),
        nlp: api_server::services::nlp::NlpService::new(&config.nlp),
        knowledge: api_server::knowledge::KnowledgeService::new("./data/knowledge"),
        prompt_builder: api_server::services::prompts::PromptBuilder::new(),
        safety: api_server::services::safety::SafetyService::new(),
    });
    
    api_server::build_app(state).await
}

#[tokio::test]
async fn test_register_validation() {
    let app = setup_app().await;
    
    let test_cases = vec![
        (r#"{}"#, StatusCode::BAD_REQUEST, "empty body"),
        (r#"{"username":"ab"}"#, StatusCode::BAD_REQUEST, "short username"),
        (r#"{"username":"valid","email":"bad","password":"12345678"}"#, StatusCode::BAD_REQUEST, "invalid email"),
        (r#"{"username":"valid","email":"a@b.com","password":"short"}"#, StatusCode::BAD_REQUEST, "short password"),
    ];
    
    for (body, expected_status, case) in test_cases {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/auth/register")
                    .header("Content-Type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), expected_status, "Failed case: {case}");
    }
}
