use axum::Router;
use metrics::counter;
use metrics_exporter_prometheus::PrometheusHandle;
use opentelemetry_sdk::trace as otel_sdk_trace;
use opentelemetry::trace::TracerProvider as _;
use std::sync::OnceLock;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

static METRICS_HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

pub fn init(config: &crate::config::TelemetryConfig) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("api_server=info,tower_http=info"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer);

    if config.enable_tracing {
        if let Some(ref endpoint) = config.otlp_endpoint {
            let exporter = opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint(endpoint)
                .build()
                .expect("Failed to build OTLP exporter");

            let provider = otel_sdk_trace::TracerProvider::builder()
                .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
                .build();

            let tracer = provider.tracer("api-server");
            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
            subscriber.with(telemetry).init();
        } else {
            subscriber.init();
        }
    } else {
        subscriber.init();
    }

    if config.enable_metrics {
        let (recorder, background) = metrics_exporter_prometheus::PrometheusBuilder::new()
            .with_http_listener(([0, 0, 0, 0], config.metrics_port))
            .build()
            .expect("Failed to build Prometheus recorder");

        tokio::spawn(background);

        let handle = recorder.handle();
        metrics::set_global_recorder(recorder)
            .expect("Failed to set global metrics recorder");

        METRICS_HANDLE
            .set(handle)
            .expect("Metrics handle already set");
    }
}

pub fn metrics_handle() -> Option<&'static PrometheusHandle> {
    METRICS_HANDLE.get()
}

pub fn metrics_router() -> Router {
    match metrics_handle() {
        Some(handle) => Router::new().route(
            "/metrics",
            axum::routing::get(move || async move { handle.render() }),
        ),
        None => Router::new(),
    }
}

pub fn record_request(method: &str, path: &str, status: u16, duration_ms: f64) {
    counter!("http_requests_total", "method" => method.to_string(), "path" => path.to_string(), "status" => status.to_string())
        .increment(1);
    metrics::histogram!("http_request_duration_ms", "method" => method.to_string(), "path" => path.to_string())
        .record(duration_ms);
}

pub fn record_inference_tokens(model: &str, tokens: u32, duration_ms: f64) {
    counter!("inference_tokens_total", "model" => model.to_string())
        .increment(tokens as u64);
    metrics::histogram!("inference_duration_ms", "model" => model.to_string())
        .record(duration_ms);
}

pub fn record_cache_hit(hit: bool) {
    counter!("cache_hits_total", "result" => if hit { "hit" } else { "miss" }.to_string())
        .increment(1);
}

pub fn record_active_users(count: usize) {
    metrics::gauge!("active_users").set(count as f64);
}

pub fn record_memory_usage(bytes: u64) {
    metrics::gauge!("memory_usage_bytes").set(bytes as f64);
}

