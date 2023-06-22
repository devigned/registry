use crate::{
    policy::{content::ContentPolicy, record::RecordPolicy},
    services::CoreService,
};
use axum::{body::Body, http::Request, Router};
use std::{path::PathBuf, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{Level, Span};
use url::Url;
use crate::contentstore::ContentStore;

pub mod v1;

#[cfg(feature = "debug")]
pub mod debug;

/// Creates the router for the API.
pub fn create_router(
    content_base_url: Url,
    core: CoreService,
    temp_dir: PathBuf,
    content_policy: Option<Arc<dyn ContentPolicy>>,
    record_policy: Option<Arc<dyn RecordPolicy>>,
    content_store: Arc<dyn ContentStore>,
) -> Router {
    let router = Router::new();
    #[cfg(feature = "debug")]
    let router = router.nest("/debug", debug::Config::new(core.clone()).into_router());
    router
        .nest(
            "/v1",
            v1::create_router(
                content_base_url,
                core,
                temp_dir,
                content_policy,
                record_policy,
                content_store,
            ),
        )
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_request(|request: &Request<Body>, _span: &Span| {
                            tracing::info!("starting {} {}", request.method(), request.uri().path())
                        })
                        .on_response(
                            DefaultOnResponse::new()
                                .level(Level::INFO)
                                .latency_unit(LatencyUnit::Micros),
                        ),
                )
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                        .allow_headers([
                            axum::http::header::CONTENT_TYPE,
                            axum::http::header::ACCEPT,
                        ]),
                ),
        )
}
