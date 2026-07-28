use axum::Router;
use std::time::Duration;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::request_id::{MakeRequestId, RequestId};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use uuid::Uuid;

use crate::SharedState;

#[derive(Clone)]
struct UuidRequestId;

impl MakeRequestId for UuidRequestId {
    fn make_request_id<B>(&mut self, _: &http::Request<B>) -> Option<RequestId> {
        let id = Uuid::now_v7().to_string().parse().ok()?;
        Some(RequestId::new(id))
    }
}

pub fn apply(state: SharedState, router: Router<SharedState>) -> Router<SharedState> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(86400));

    let trace = TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .include_headers(true)
                .level(Level::INFO),
        )
        .on_response(
            DefaultOnResponse::new()
                .include_headers(true)
                .level(Level::INFO),
        );

    let compression = CompressionLayer::new()
        .gzip(true)
        .br(true)
        .no_zstd()
        .quality(tower_http::compression::CompressionLevel::Best);

    let request_id = tower_http::request_id::SetRequestIdLayer::new(
        http::header::HeaderName::from_static("x-request-id"),
        UuidRequestId {},
    );

    let body_limit = RequestBodyLimitLayer::new(
        state.config.security.max_request_size_bytes,
    );

    let catch_panic = CatchPanicLayer::new();

    router
        .layer(catch_panic)
        .layer(compression)
        .layer(cors)
        .layer(body_limit)
        .layer(request_id)
        .layer(trace)
}
