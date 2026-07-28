use eyre::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::sync::OnceLock;

static INSTANCE: OnceLock<Config> = OnceLock::new();

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub vector_db: VectorDbConfig,
    pub search: SearchConfig,
    pub auth: AuthConfig,
    pub inference: InferenceConfig,
    pub telemetry: TelemetryConfig,
    pub security: SecurityConfig,
    pub upload: UploadConfig,
    pub nlp: NlpConfig,
    pub knowledge: KnowledgeConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub host: String,
    pub port: u16,
    pub environment: Environment,
    pub log_level: String,
    pub shutdown_timeout_secs: u64,
    pub max_body_size: usize,
    pub cors_origins: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
    pub pool_health_check_interval_secs: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheConfig {
    pub redis_url: String,
    pub pool_size: u32,
    pub default_ttl_secs: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorDbConfig {
    pub url: String,
    pub api_key: String,
    pub collection_name: String,
    pub vector_size: u64,
    pub similarity_threshold: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchConfig {
    pub url: String,
    pub api_key: String,
    pub index_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expiration_secs: u64,
    pub refresh_token_expiration_secs: u64,
    pub bcrypt_cost: u32,
    pub max_login_attempts: u32,
    pub lockout_duration_secs: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InferenceConfig {
    pub model: String,
    pub api_url: String,
    pub api_key: String,
    pub max_tokens: u32,
    pub default_temperature: f32,
    pub max_context_length: u32,
    pub streaming: bool,
    pub request_timeout_secs: u64,
    pub max_concurrent_requests: usize,
    pub cpu_threads: u32,
    pub use_mlock: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enable_tracing: bool,
    pub enable_metrics: bool,
    pub otlp_endpoint: Option<String>,
    pub metrics_port: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub rate_limit_requests_per_minute: u32,
    pub rate_limit_burst_size: u32,
    pub max_request_size_bytes: usize,
    pub allowed_content_types: Vec<String>,
    pub enable_request_validation: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UploadConfig {
    pub max_file_size: u64,
    pub allowed_extensions: Vec<String>,
    pub storage_path: PathBuf,
    pub chunk_size: usize,
    pub enable_virus_scan: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NlpConfig {
    pub enable_language_detection: bool,
    pub enable_urdu_normalization: bool,
    pub enable_roman_urdu: bool,
    pub enable_code_switch: bool,
    pub roman_urdu_dict_path: Option<String>,
    pub punjabi_dict_path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeConfig {
    pub data_dir: PathBuf,
    pub enable_auto_ingest: bool,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
    pub min_confidence: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Staging,
    Production,
    Test,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Development => write!(f, "development"),
            Self::Staging => write!(f, "staging"),
            Self::Production => write!(f, "production"),
            Self::Test => write!(f, "test"),
        }
    }
}

impl Environment {
    pub fn is_production(&self) -> bool {
        matches!(self, Self::Production)
    }

    pub fn is_development(&self) -> bool {
        matches!(self, Self::Development)
    }
}

impl Config {
    fn init_from_env() -> Result<Config> {
        dotenvy::dotenv().ok();

            let logical_cpus = std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4);

            Ok(Self {
                app: AppConfig {
                    name: env::var("APP_NAME").unwrap_or_else(|_| "MAR-1.0".into()),
                    version: env::var("APP_VERSION").unwrap_or_else(|_| "1.0.0".into()),
                    host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
                    port: env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080),
                    environment: match env::var("ENVIRONMENT").as_deref() {
                        Ok("production") => Environment::Production,
                        Ok("staging") => Environment::Staging,
                        Ok("test") => Environment::Test,
                        _ => Environment::Development,
                    },
                    log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),
                    shutdown_timeout_secs: env::var("SHUTDOWN_TIMEOUT_SECS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(30),
                    max_body_size: env::var("MAX_BODY_SIZE")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(10 * 1024 * 1024),
                    cors_origins: env::var("CORS_ORIGINS")
                        .unwrap_or_else(|_| "*".into())
                        .split(',')
                        .map(String::from)
                        .collect(),
                },
                database: DatabaseConfig {
                    url: env::var("DATABASE_URL")
                        .unwrap_or_else(|_| "postgres://mar:mar@localhost:5432/mar".into()),
                    max_connections: env::var("DB_MAX_CONNECTIONS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(100),
                    min_connections: env::var("DB_MIN_CONNECTIONS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(10),
                    acquire_timeout_secs: env::var("DB_ACQUIRE_TIMEOUT_SECS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(30),
                    idle_timeout_secs: env::var("DB_IDLE_TIMEOUT_SECS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(600),
                    max_lifetime_secs: env::var("DB_MAX_LIFETIME_SECS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(1800),
                    pool_health_check_interval_secs: env::var("DB_HEALTH_CHECK_INTERVAL_SECS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(30),
                },
                cache: CacheConfig {
                    redis_url: env::var("REDIS_URL")
                        .unwrap_or_else(|_| "redis://localhost:6379".into()),
                    pool_size: env::var("REDIS_POOL_SIZE")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(32),
                    default_ttl_secs: env::var("REDIS_DEFAULT_TTL_SECS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(3600),
                },
                vector_db: VectorDbConfig {
                    url: env::var("QDRANT_URL")
                        .unwrap_or_else(|_| "http://localhost:6333".into()),
                    api_key: env::var("QDRANT_API_KEY").unwrap_or_default(),
                    collection_name: env::var("QDRANT_COLLECTION")
                        .unwrap_or_else(|_| "mar_knowledge".into()),
                    vector_size: env::var("VECTOR_SIZE")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(1024),
                    similarity_threshold: env::var("SIMILARITY_THRESHOLD")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(0.75),
                },
                search: SearchConfig {
                    url: env::var("MEILISEARCH_URL")
                        .unwrap_or_else(|_| "http://localhost:7700".into()),
                    api_key: env::var("MEILISEARCH_API_KEY").unwrap_or_default(),
                    index_name: env::var("MEILISEARCH_INDEX")
                        .unwrap_or_else(|_| "mar_knowledge".into()),
                },
                auth: AuthConfig {
                    jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| {
                        tracing::warn!("JWT_SECRET not set - using insecure default");
                        "change-me-in-production-please".into()
                    }),
                    jwt_expiration_secs: env::var("JWT_EXPIRATION_SECS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(86400),
                    refresh_token_expiration_secs: env::var("REFRESH_TOKEN_EXPIRATION_SECS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(2592000),
                    bcrypt_cost: env::var("BCRYPT_COST")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(12),
                    max_login_attempts: env::var("MAX_LOGIN_ATTEMPTS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(5),
                    lockout_duration_secs: env::var("LOCKOUT_DURATION_SECS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(900),
                },
                inference: InferenceConfig {
                    model: env::var("INFERENCE_MODEL")
                        .unwrap_or_else(|_| "mar-8b".into()),
                    api_url: env::var("INFERENCE_API_URL")
                        .unwrap_or_else(|_| "http://localhost:8081/v1".into()),
                    api_key: env::var("INFERENCE_API_KEY").unwrap_or_default(),
                    max_tokens: env::var("MAX_TOKENS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(4096),
                    default_temperature: env::var("DEFAULT_TEMPERATURE")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(0.7),
                    max_context_length: env::var("MAX_CONTEXT_LENGTH")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(32768),
                    streaming: env::var("STREAMING").ok().map(|v| v == "true").unwrap_or(true),
                    request_timeout_secs: env::var("INFERENCE_REQUEST_TIMEOUT_SECS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(120),
                    max_concurrent_requests: env::var("MAX_CONCURRENT_REQUESTS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(4),
                    cpu_threads: env::var("CPU_THREADS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(logical_cpus.saturating_sub(2).max(2) as u32),
                    use_mlock: env::var("USE_MLOCK")
                        .ok().map(|v| v == "true").unwrap_or(true),
                },
                telemetry: TelemetryConfig {
                    enable_tracing: env::var("ENABLE_TRACING")
                        .ok().map(|v| v == "true").unwrap_or(true),
                    enable_metrics: env::var("ENABLE_METRICS")
                        .ok().map(|v| v == "true").unwrap_or(true),
                    otlp_endpoint: env::var("OTLP_ENDPOINT").ok(),
                    metrics_port: env::var("METRICS_PORT")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(9090),
                },
                security: SecurityConfig {
                    rate_limit_requests_per_minute: env::var("RATE_LIMIT_REQUESTS")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(60),
                    rate_limit_burst_size: env::var("RATE_LIMIT_BURST")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(100),
                    max_request_size_bytes: env::var("MAX_REQUEST_SIZE")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(10 * 1024 * 1024),
                    allowed_content_types: vec![
                        "application/json".into(),
                        "text/plain".into(),
                        "multipart/form-data".into(),
                    ],
                    enable_request_validation: true,
                },
                upload: UploadConfig {
                    max_file_size: env::var("MAX_FILE_SIZE")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(50 * 1024 * 1024),
                    allowed_extensions: vec![
                        "pdf".into(), "docx".into(), "xlsx".into(),
                        "txt".into(), "md".into(), "csv".into(),
                        "png".into(), "jpg".into(), "jpeg".into(),
                    ],
                    storage_path: PathBuf::from(
                        env::var("UPLOAD_PATH").unwrap_or_else(|_| "./uploads".into())
                    ),
                    chunk_size: 8192,
                    enable_virus_scan: env::var("ENABLE_VIRUS_SCAN")
                        .ok().map(|v| v == "true").unwrap_or(false),
                },
                nlp: NlpConfig {
                    enable_language_detection: env::var("ENABLE_LANG_DETECTION")
                        .ok().map(|v| v == "true").unwrap_or(true),
                    enable_urdu_normalization: env::var("ENABLE_URDU_NORMALIZATION")
                        .ok().map(|v| v == "true").unwrap_or(true),
                    enable_roman_urdu: env::var("ENABLE_ROMAN_URDU")
                        .ok().map(|v| v == "true").unwrap_or(true),
                    enable_code_switch: env::var("ENABLE_CODE_SWITCH")
                        .ok().map(|v| v == "true").unwrap_or(true),
                    roman_urdu_dict_path: env::var("ROMAN_URDU_DICT_PATH").ok(),
                    punjabi_dict_path: env::var("PUNJABI_DICT_PATH").ok(),
                },
                knowledge: KnowledgeConfig {
                    data_dir: PathBuf::from(
                        env::var("KNOWLEDGE_DATA_DIR").unwrap_or_else(|_| "./data/knowledge".into())
                    ),
                    enable_auto_ingest: env::var("ENABLE_AUTO_INGEST")
                        .ok().map(|v| v == "true").unwrap_or(true),
                    chunk_size: env::var("KNOWLEDGE_CHUNK_SIZE")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(512),
                    chunk_overlap: env::var("KNOWLEDGE_CHUNK_OVERLAP")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(64),
                    min_confidence: env::var("KNOWLEDGE_MIN_CONFIDENCE")
                        .ok().and_then(|v| v.parse().ok()).unwrap_or(0.7),
                },
            })
    }

    pub fn load() -> Result<&'static Self> {
        Ok(INSTANCE.get_or_init(|| {
            Self::init_from_env().expect("Failed to initialize config from environment")
        }))
    }
}
