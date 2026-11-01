use crate::{
    application::boards::errors::BoardAppError,
    domain::{
        boards::{ports::BoardRepo, Board},
        teams::TeamId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct ListBoardsInput {
    pub team_id: Uuid,
}

pub struct ListBoardsUseCase {
    board_repo: Arc<dyn BoardRepo>,
}

impl ListBoardsUseCase {
    pub fn new(board_repo: Arc<dyn BoardRepo>) -> Self {
        Self { board_repo }
    }

    pub async fn execute(&self, input: ListBoardsInput) -> Result<Vec<Board>, BoardAppError> {
        let boards = self.board_repo.list(TeamId::from(input.team_id)).await?;
        Ok(boards)
    }
}
