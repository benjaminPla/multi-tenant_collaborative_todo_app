use crate::domain::{
    boards::{Board, BoardId},
    teams::TeamId,
    BoxError,
};
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum BoardRepoError {
    #[error("board not found")]
    NotFound,
    #[error(transparent)]
    Unexpected(BoxError),
}

#[async_trait]
pub trait BoardRepo: Send + Sync {
    async fn create(&self, board: &Board)                    -> Result<Board, BoardRepoError>;
    async fn delete(&self, team_id: TeamId, id: BoardId)     -> Result<(), BoardRepoError>;
    async fn find_by_id(&self, team_id: TeamId, id: BoardId) -> Result<Option<Board>, BoardRepoError>;
    async fn list(&self, team_id: TeamId)                    -> Result<Vec<Board>, BoardRepoError>;
    async fn update(&self, board: &Board)                    -> Result<Board, BoardRepoError>;
}
