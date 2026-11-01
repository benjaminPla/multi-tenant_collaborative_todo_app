use crate::domain::{
    hash::HasherService,
    sessions::ports::SessionRepo,
    users::ports::UserRepo,
};
use chrono::Duration;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub hasher_service: Arc<dyn HasherService>,
    pub session_repo:   Arc<dyn SessionRepo>,
    pub session_ttl:    Duration,
    pub user_repo:      Arc<dyn UserRepo>,
}
