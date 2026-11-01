use crate::domain::{
    authz::ports::Pip,
    boards::ports::BoardRepo,
    hash::HasherService,
    sessions::ports::SessionRepo,
    teams::ports::TeamRepo,
    todos::ports::{TodoEvents, TodoRepo},
    users::ports::UserRepo,
};
use chrono::Duration;
use std::sync::Arc;
use tokio::sync::{watch, Semaphore};

#[derive(Clone)]
pub struct AppState {
    pub board_repo:     Arc<dyn BoardRepo>,
    pub hasher_service: Arc<dyn HasherService>,
    pub pip:            Arc<dyn Pip>,
    pub session_repo:   Arc<dyn SessionRepo>,
    pub session_ttl:    Duration,
    pub shutdown:       watch::Receiver<bool>,
    pub sse_permits:    Arc<Semaphore>,
    pub team_repo:      Arc<dyn TeamRepo>,
    pub todo_events:    Arc<dyn TodoEvents>,
    pub todo_repo:      Arc<dyn TodoRepo>,
    pub user_repo:      Arc<dyn UserRepo>,
}
