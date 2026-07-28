use std::sync::Arc;
use tracing::info;

mod config;
mod engine;
mod api;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "inference=info".into()),
        )
        .init();

    let config = config::InferenceConfig::from_env()?;

    info!(
        model = %config.model_path,
        threads = config.n_threads,
        ctx = config.n_ctx,
        "Starting MAR inference server with llama.cpp GGUF"
    );

    let engine = engine::InferenceEngine::new(config.clone())?;
    let engine = Arc::new(engine);

    let app = api::router(engine);

    let addr = format!("{}:{}", config.host, config.port);
    info!("Inference server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    info!("Inference server shutting down");
}
