use crate::config::Config;
use axum::{extract::Request, response::Response, ServiceExt};
use std::{convert::Infallible, net::SocketAddr};
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::watch,
};
use tower::Service;

pub async fn serve<S>(config: &Config, service: S, shutdown: watch::Sender<bool>)
where
    S: Service<Request, Response = Response, Error = Infallible, Future: Send> + Clone + Send + 'static,
{
    let address  = SocketAddr::new(config.host, config.port);
    let listener = tokio::net::TcpListener::bind(address).await.expect("Failed to bind");
    tracing::info!(%address, "listening");
    axum::serve(listener, ServiceExt::<Request>::into_make_service(service))
        .with_graceful_shutdown(async move {
            signal_received().await;
            tracing::info!("shutting down");
            let _ = shutdown.send(true);
        })
        .await
        .expect("Server failed");
}

// as PID 1 in a container SIGTERM is ignored without a handler
async fn signal_received() {
    let mut terminate = signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = terminate.recv()        => {}
    }
}
