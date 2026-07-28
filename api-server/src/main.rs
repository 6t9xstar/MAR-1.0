use std::sync::Arc;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config = api_server::config::Config::load()?;

    api_server::telemetry::init(&config.telemetry);

    info!(
        name = %config.app.name,
        version = %config.app.version,
        env = %config.app.environment,
        "Starting MAR 1.0 API server"
    );

    let pool = api_server::db::init_pool(&config.database).await?;
    api_server::db::migrations::run(&pool).await?;
    info!("Database migrations complete");

    let cache = api_server::db::init_cache(&config.cache).await?;
    let vector_store = api_server::db::init_vector_store(&config.vector_db).await?;
    let search = api_server::db::init_search(&config.search).await?;

    let inference = api_server::services::inference::InferenceClient::new(config.inference.clone());
    let memory = api_server::services::memory::MemoryService::new(
        pool.clone(),
        Arc::new(vector_store.clone()),
        &config,
    );

    let nlp = api_server::services::nlp::NlpService::new(&config.nlp);
    let knowledge = api_server::knowledge::KnowledgeService::new(
        config.knowledge.data_dir.to_str().unwrap_or("./data/knowledge"),
    );
    let prompt_builder = api_server::services::prompts::PromptBuilder::new();
    let safety = api_server::services::safety::SafetyService::new();

    if config.knowledge.enable_auto_ingest {
        info!("Auto-ingesting knowledge base");
        knowledge.ingest_all().await;
        info!("Knowledge base ingested: {} skills loaded", knowledge.registry.len());
    }

    let state = Arc::new(api_server::AppState {
        config: config.clone(),
        db: pool,
        cache,
        vector_store: Arc::new(vector_store),
        search,
        inference,
        memory,
        nlp,
        knowledge,
        prompt_builder,
        safety,
    });

    let app = api_server::build_app(state.clone()).await;
    let metrics_app = api_server::telemetry::metrics_router();

    let addr = format!("{}:{}", config.app.host, config.app.port);
    info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app.merge(metrics_app))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .ok()
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { info!("Received Ctrl+C, shutting down"); }
        _ = terminate => { info!("Received SIGTERM, shutting down"); }
    }
}
