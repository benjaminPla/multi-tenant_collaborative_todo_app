use crate::domain::{
    boards::BoardId,
    teams::TeamId,
    todos::{Todo, TodoEvent, TodoId},
    BoxError,
};
use async_trait::async_trait;
use std::pin::Pin;
use tokio_stream::Stream;

pub type TodoFeed = Pin<Box<dyn Stream<Item = TodoFeedItem> + Send>>;

pub enum TodoFeedItem {
    Event(TodoEvent),
    Lagged,
}

pub trait TodoEvents: Send + Sync {
    fn publish(&self, event: TodoEvent);
    fn subscribe(&self) -> TodoFeed;
}

#[derive(Debug, thiserror::Error)]
pub enum TodoRepoError {
    #[error("assignee is not a member of the team")]
    AssigneeNotMember,
    #[error("board not found")]
    BoardNotFound,
    #[error("todo not found")]
    NotFound,
    #[error(transparent)]
    Unexpected(BoxError),
    #[error("todo was changed by someone else")]
    VersionConflict,
}

#[async_trait]
pub trait TodoRepo: Send + Sync {
    async fn create(&self, todo: &Todo)                                        -> Result<Todo, TodoRepoError>;
    async fn delete(&self, team_id: TeamId, board_id: BoardId, id: TodoId)     -> Result<Todo, TodoRepoError>;
    async fn find_by_id(&self, team_id: TeamId, board_id: BoardId, id: TodoId) -> Result<Option<Todo>, TodoRepoError>;
    async fn list(&self, team_id: TeamId, board_id: BoardId)                   -> Result<Vec<Todo>, TodoRepoError>;
    async fn update(&self, todo: &Todo)                                        -> Result<Todo, TodoRepoError>;
}
