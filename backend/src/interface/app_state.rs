use crate::domain::{
    authz::ports::Pip,
    boards::ports::BoardRepo,
    hash::HasherService,
    sessions::ports::SessionRepo,
    teams::ports::TeamRepo,
    users::ports::UserRepo,
};
use chrono::Duration;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub board_repo:     Arc<dyn BoardRepo>,
    pub hasher_service: Arc<dyn HasherService>,
    pub pip:            Arc<dyn Pip>,
    pub session_repo:   Arc<dyn SessionRepo>,
    pub session_ttl:    Duration,
    pub team_repo:      Arc<dyn TeamRepo>,
    pub user_repo:      Arc<dyn UserRepo>,
}
