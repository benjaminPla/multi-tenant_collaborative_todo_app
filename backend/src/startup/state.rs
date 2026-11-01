use crate::{
    config::Config,
    infrastructure::{
        argon2_hasher_service::Argon2HasherService,
        broadcast_todo_events::BroadcastTodoEvents,
        pg_board_repo::PgBoardRepo,
        pg_pip::PgPip,
        pg_session_repo::PgSessionRepo,
        pg_team_repo::PgTeamRepo,
        pg_todo_repo::PgTodoRepo,
        pg_user_repo::PgUserRepo,
    },
    interface::app_state::AppState,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::{watch, Semaphore};

pub fn build(pool: PgPool, config: &Config, shutdown: watch::Receiver<bool>) -> AppState {
    AppState {
        board_repo:     Arc::new(PgBoardRepo::new(pool.clone())),
        hasher_service: Arc::new(Argon2HasherService::new()),
        pip:            Arc::new(PgPip::new(pool.clone())),
        session_repo:   Arc::new(PgSessionRepo::new(pool.clone())),
        session_ttl:    chrono::Duration::seconds(config.session_ttl_secs.into()),
        shutdown,
        sse_permits:    Arc::new(Semaphore::new(config.sse_max_connections)),
        team_repo:      Arc::new(PgTeamRepo::new(pool.clone())),
        todo_events:    Arc::new(BroadcastTodoEvents::new()),
        todo_repo:      Arc::new(PgTodoRepo::new(pool.clone())),
        user_repo:      Arc::new(PgUserRepo::new(pool)),
    }
}
