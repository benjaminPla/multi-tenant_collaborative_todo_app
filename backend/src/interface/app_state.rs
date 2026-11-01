use crate::domain::{hash::HasherService, users::ports::UserRepo};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub hasher_service: Arc<dyn HasherService>,
    pub user_repo:      Arc<dyn UserRepo>,
}
