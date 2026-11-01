mod application;
mod config;
mod domain;
mod infrastructure;
mod interface;

use config::Config;
use infrastructure::{
    argon2_hasher_service::Argon2HasherService,
    broadcast_todo_events::BroadcastTodoEvents,
    pg_board_repo::PgBoardRepo,
    pg_pip::PgPip,
    pg_session_repo::PgSessionRepo,
    pg_team_repo::PgTeamRepo,
    pg_todo_repo::PgTodoRepo,
    pg_user_repo::PgUserRepo,
};
use interface::{app_state::AppState, router::app};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    signal::unix::{signal, SignalKind},
    sync::watch,
};

#[tokio::main]
async fn main() {
    let config = envy::from_env::<Config>().expect("Failed to load env vars");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(config.db_min_connections)
        .acquire_timeout(Duration::from_secs(config.db_acquire_timeout_secs))
        .idle_timeout(Duration::from_secs(config.db_idle_timeout_secs))
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    let app_state = AppState {
        board_repo:     Arc::new(PgBoardRepo::new(pool.clone())),
        hasher_service: Arc::new(Argon2HasherService::new()),
        pip:            Arc::new(PgPip::new(pool.clone())),
        session_repo:   Arc::new(PgSessionRepo::new(pool.clone())),
        session_ttl:    chrono::Duration::seconds(config.session_ttl_secs.into()),
        shutdown:       shutdown_rx,
        team_repo:      Arc::new(PgTeamRepo::new(pool.clone())),
        todo_events:    Arc::new(BroadcastTodoEvents::new()),
        todo_repo:      Arc::new(PgTodoRepo::new(pool.clone())),
        user_repo:      Arc::new(PgUserRepo::new(pool.clone())),
    };

    let app = app(app_state);

    let address  = SocketAddr::new(config.host, config.port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(async move {
            shutdown_signal().await;
            let _ = shutdown_tx.send(true);
        })
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let mut terminate = signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {}
        _ = terminate.recv()        => {}
    }
}
