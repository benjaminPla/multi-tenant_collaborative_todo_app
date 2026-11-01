use crate::{config::Config, interface::middleware};
use axum::{
    error_handling::HandleErrorLayer,
    extract::Request,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Router,
};
use std::{convert::Infallible, time::Duration};
use tower::{limit::ConcurrencyLimitLayer, load_shed::LoadShedLayer, Service, ServiceBuilder};
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    limit::RequestBodyLimitLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    timeout::{RequestBodyTimeoutLayer, TimeoutLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// around the whole router: Router::layer would give every route its own concurrency limit
pub fn service(
    router: Router,
    config: &Config,
) -> impl Service<Request, Response = Response, Error = Infallible, Future: Send> + Clone + Send + 'static {
    ServiceBuilder::new()
        .map_response(IntoResponse::into_response)
        .layer(CatchPanicLayer::custom(middleware::panicked))
        .layer(SetSensitiveHeadersLayer::new([header::COOKIE, header::SET_COOKIE]))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_failure(())
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(CompressionLayer::new())
        .layer(HandleErrorLayer::new(middleware::overloaded))
        .layer(LoadShedLayer::new())
        .layer(ConcurrencyLimitLayer::new(config.http_concurrency_limit))
        .layer(TimeoutLayer::with_status_code(StatusCode::SERVICE_UNAVAILABLE, Duration::from_secs(config.http_timeout_secs)))
        .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(config.http_body_timeout_secs)))
        .layer(RequestBodyLimitLayer::new(config.http_body_limit_bytes))
        .service(router)
}
