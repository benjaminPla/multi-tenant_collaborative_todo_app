use crate::domain::{
    authz::ports::Pip,
    hash::HasherService,
    sessions::ports::SessionRepo,
    teams::ports::TeamRepo,
    users::ports::UserRepo,
};
use chrono::Duration;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub hasher_service: Arc<dyn HasherService>,
    pub pip:            Arc<dyn Pip>,
    pub session_repo:   Arc<dyn SessionRepo>,
    pub session_ttl:    Duration,
    pub team_repo:      Arc<dyn TeamRepo>,
    pub user_repo:      Arc<dyn UserRepo>,
}
