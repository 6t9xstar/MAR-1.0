use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InferenceConfig {
    pub model_path: String,
    pub model_alias: String,
    pub tokenizer_path: Option<String>,
    pub n_gpu_layers: u32,
    pub n_ctx: u32,
    pub n_batch: u32,
    pub n_threads: u32,
    pub n_predict: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: u32,
    pub repeat_penalty: f32,
    pub seed: u64,
    pub host: String,
    pub port: u16,
}

impl InferenceConfig {
    pub fn from_env() -> eyre::Result<Self> {
        let logical_cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        Ok(Self {
            model_path: env::var("MODEL_PATH").unwrap_or_else(|_| {
                "models/MAR-8B-Q4_K_M.gguf".into()
            }),
            model_alias: env::var("MODEL_ALIAS").unwrap_or_else(|_| "mar-8b".into()),
            tokenizer_path: env::var("TOKENIZER_PATH").ok(),
            n_gpu_layers: env::var("N_GPU_LAYERS")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(0),
            n_ctx: env::var("N_CTX")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(32768),
            n_batch: env::var("N_BATCH")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(512),
            n_threads: env::var("N_THREADS")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(logical_cpus.saturating_sub(2).max(2)),
            n_predict: env::var("N_PREDICT")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(4096),
            temperature: env::var("TEMPERATURE")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(0.7),
            top_p: env::var("TOP_P")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(0.9),
            top_k: env::var("TOP_K")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(40),
            repeat_penalty: env::var("REPEAT_PENALTY")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(1.1),
            seed: env::var("SEED")
                .ok().and_then(|v| v.parse().ok()).unwrap_or(42),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: env::var("PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(8081),
        })
    }
}
