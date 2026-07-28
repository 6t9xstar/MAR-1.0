use crate::config::InferenceConfig;
use llama_cpp_2::context::params::ContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::ModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::token::data_array::LlamaTokenDataArray;
use llama_cpp_2::token::LlamaToken;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, warn};

pub struct InferenceEngine {
    config: InferenceConfig,
    backend: Arc<LlamaBackend>,
    model: Arc<LlamaModel>,
    context: Arc<Mutex<LlamaContext<'static>>>,
}

impl InferenceEngine {
    pub fn new(config: InferenceConfig) -> eyre::Result<Self> {
        info!(
            model = %config.model_path,
            threads = config.n_threads,
            ctx = config.n_ctx,
            "Initializing inference engine with llama.cpp"
        );

        let backend = LlamaBackend::init()?;
        let model_params = ModelParams::default()
            .with_n_gpu_layers(config.n_gpu_layers);

        let model = LlamaModel::load_from_file(&backend, &config.model_path, &model_params)?;

        let ctx_params = ContextParams::default()
            .with_n_ctx(Some(config.n_ctx))
            .with_n_batch(config.n_batch)
            .with_n_threads(config.n_threads)
            .with_n_threads_batch(config.n_threads);

        let context = model.new_context(&backend, &ctx_params)?;

        info!("Model loaded: {}", model.name());

        Ok(Self {
            config,
            backend: Arc::new(backend),
            model: Arc::new(model),
            context: Arc::new(Mutex::new(context)),
        })
    }

    pub async fn generate(
        &self,
        prompt: &str,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> eyre::Result<String> {
        let max_tokens = max_tokens.unwrap_or(self.config.n_predict);
        let temperature = temperature.unwrap_or(self.config.temperature);
        let mut ctx = self.context.lock().await;

        let tokens = self.model.str_to_token(&prompt, true)?;
        let n_tokens = tokens.len() as i32;

        if n_tokens + max_tokens as i32 > self.config.n_ctx as i32 {
            warn!(
                prompt_tokens = n_tokens,
                max_tokens = max_tokens,
                n_ctx = self.config.n_ctx,
                "Prompt + response exceeds context window, will truncate"
            );
        }

        ctx.clear_kv_cache();
        let mut batch = LlamaBatch::new(n_tokens as u32 + max_tokens, 1, 1)?;
        for (i, &token) in tokens.iter().enumerate() {
            batch.add(token, i as i32, &[0], i == tokens.len() - 1)?;
        }
        ctx.decode(&mut batch)?;

        let mut output = String::new();
        let mut last_n_tokens: Vec<LlamaToken> = tokens.iter().copied().collect();
        let eos = self.model.token_eos();

        let mut sparams = self.model.sampler_params()?;
        sparams.set_temperature(temperature);
        sparams.set_top_p(self.config.top_p);
        sparams.set_top_k(self.config.top_k);
        sparams.set_repeat_penalty(self.config.repeat_penalty);

        for i in 0..max_tokens {
            let candidates = ctx.candidates();
            let mut data = LlamaTokenDataArray::from_candidates(candidates, last_n_tokens.len());
            let token = data.sample_token(&sparams, &last_n_tokens)?;
            ctx.update(token);

            if token == eos {
                break;
            }

            let piece = self.model.token_to_str(token)?;
            output.push_str(&piece);

            last_n_tokens.push(token);
            if last_n_tokens.len() > self.config.n_ctx as usize {
                last_n_tokens.remove(0);
            }

            let mut batch = LlamaBatch::new(1, 0, 1)?;
            batch.add(token, (n_tokens + i as i32), &[0], true)?;
            ctx.decode(&mut batch)?;
        }

        Ok(output)
    }

    pub async fn generate_stream(
        &self,
        prompt: &str,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> eyre::Result<tokio::sync::mpsc::Receiver<String>> {
        let max_tokens = max_tokens.unwrap_or(self.config.n_predict);
        let temperature = temperature.unwrap_or(self.config.temperature);

        let (tx, rx) = tokio::sync::mpsc::channel::<String>(256);
        let model = self.model.clone();
        let context = self.context.clone();
        let config = self.config.clone();

        let prompt = prompt.to_string();

        tokio::spawn(async move {
            let mut ctx = context.lock().await;

            let tokens = match model.str_to_token(&prompt, true) {
                Ok(t) => t,
                Err(e) => {
                    let _ = tx.send(format!("[Error: {e}]")).await;
                    return;
                }
            };
            let n_tokens = tokens.len() as i32;

            ctx.clear_kv_cache();
            let mut batch = match LlamaBatch::new(n_tokens as u32 + max_tokens, 1, 1) {
                Ok(b) => b,
                Err(e) => {
                    let _ = tx.send(format!("[Error: {e}]")).await;
                    return;
                }
            };
            for (i, &token) in tokens.iter().enumerate() {
                if batch.add(token, i as i32, &[0], i == tokens.len() - 1).is_err() {
                    break;
                }
            }
            if ctx.decode(&mut batch).is_err() {
                return;
            }

            let mut last_n_tokens: Vec<LlamaToken> = tokens.iter().copied().collect();
            let eos = model.token_eos();

            let mut sparams = match model.sampler_params() {
                Ok(s) => s,
                Err(_) => return,
            };
            sparams.set_temperature(temperature);
            sparams.set_top_p(config.top_p);
            sparams.set_top_k(config.top_k);
            sparams.set_repeat_penalty(config.repeat_penalty);

            for i in 0..max_tokens {
                let candidates = ctx.candidates();
                let mut data = LlamaTokenDataArray::from_candidates(candidates, last_n_tokens.len());
                let token = data.sample_token(&sparams, &last_n_tokens).unwrap_or(eos);
                ctx.update(token);

                if token == eos {
                    break;
                }

                let piece = model.token_to_str(token).unwrap_or_default();
                if tx.send(piece.to_string()).await.is_err() {
                    break;
                }

                last_n_tokens.push(token);
                if last_n_tokens.len() > config.n_ctx as usize {
                    last_n_tokens.remove(0);
                }

                let mut batch = LlamaBatch::new(1, 0, 1).unwrap();
                if batch.add(token, n_tokens + i as i32, &[0], true).is_ok() {
                    let _ = ctx.decode(&mut batch);
                }
            }
        });

        Ok(rx)
    }

    pub fn model_name(&self) -> String {
        self.model.name().to_string()
    }

    pub fn config(&self) -> &InferenceConfig {
        &self.config
    }
}
