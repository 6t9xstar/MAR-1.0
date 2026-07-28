pub mod config;
pub mod db;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod telemetry;
pub mod knowledge;

use axum::Router;
use std::sync::Arc;

pub struct AppState {
    pub config: config::Config,
    pub db: db::Pool,
    pub cache: db::CachePool,
    pub vector_store: Arc<db::VectorStore>,
    pub search: db::SearchClient,
    pub inference: services::inference::InferenceClient,
    pub memory: services::memory::MemoryService,
    pub nlp: services::nlp::NlpService,
    pub knowledge: knowledge::KnowledgeService,
    pub prompt_builder: services::prompts::PromptBuilder,
    pub safety: services::safety::SafetyService,
}

pub type SharedState = Arc<AppState>;

pub async fn build_app(state: SharedState) -> Router {
    let openapi = routes::docs::openapi();
    let docs_router = routes::docs::router(openapi);

    let router = Router::new()
        .merge(routes::chat::router())
        .merge(routes::auth::router())
        .merge(routes::memory::router())
        .merge(routes::documents::router())
        .merge(routes::tools::router())
        .merge(routes::admin::router())
        .merge(docs_router);

    middleware::layers::apply(state.clone(), router).with_state(state)
}
