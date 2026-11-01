use crate::{
    application::boards::errors::BoardAppError,
    domain::{
        boards::{ports::BoardRepo, BoardId},
        teams::TeamId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteBoardInput {
    pub id:      Uuid,
    pub team_id: Uuid,
}

pub struct DeleteBoardUseCase {
    board_repo: Arc<dyn BoardRepo>,
}

impl DeleteBoardUseCase {
    pub fn new(board_repo: Arc<dyn BoardRepo>) -> Self {
        Self { board_repo }
    }

    pub async fn execute(&self, input: DeleteBoardInput) -> Result<(), BoardAppError> {
        self.board_repo.delete(TeamId::from(input.team_id), BoardId::from(input.id)).await?;
        Ok(())
    }
}
