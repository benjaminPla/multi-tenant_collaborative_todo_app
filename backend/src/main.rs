mod application;
mod config;
mod domain;
mod infrastructure;
mod interface;

use axum::Router;
use config::Config;
use infrastructure::{
    argon2_hasher_service::Argon2HasherService,
    pg_session_repo::PgSessionRepo,
    pg_team_repo::PgTeamRepo,
    pg_user_repo::PgUserRepo,
};
use interface::{
    app_state::AppState,
    authn::router::authn_router,
    teams::router::teams_router,
    users::router::users_router,
};
use std::{net::SocketAddr, sync::Arc, time::Duration};

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

    let app_state = AppState {
        hasher_service: Arc::new(Argon2HasherService::new()),
        session_repo:   Arc::new(PgSessionRepo::new(pool.clone())),
        session_ttl:    chrono::Duration::seconds(config.session_ttl_secs.into()),
        team_repo:      Arc::new(PgTeamRepo::new(pool.clone())),
        user_repo:      Arc::new(PgUserRepo::new(pool.clone())),
    };

    let app = Router::new()
        .nest("/auth", authn_router())
        .nest("/teams", teams_router())
        .nest("/users", users_router())
        .with_state(app_state);

    let address  = SocketAddr::from(([127, 0, 0, 1], config.port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
