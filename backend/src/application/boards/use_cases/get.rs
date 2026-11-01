use crate::{
    application::boards::errors::BoardAppError,
    domain::{
        boards::{
            ports::{BoardRepo, BoardRepoError},
            Board,
            BoardId,
        },
        teams::TeamId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetBoardInput {
    pub id:      Uuid,
    pub team_id: Uuid,
}

pub struct GetBoardUseCase {
    board_repo: Arc<dyn BoardRepo>,
}

impl GetBoardUseCase {
    pub fn new(board_repo: Arc<dyn BoardRepo>) -> Self {
        Self { board_repo }
    }

    pub async fn execute(&self, input: GetBoardInput) -> Result<Board, BoardAppError> {
        let board = self.board_repo
            .find_by_id(TeamId::from(input.team_id), BoardId::from(input.id))
            .await?
            .ok_or(BoardRepoError::NotFound)?;
        Ok(board)
    }
}
