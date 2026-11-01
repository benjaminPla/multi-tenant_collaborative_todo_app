use crate::{
    application::boards::errors::BoardAppError,
    domain::{
        boards::{
            ports::{BoardRepo, BoardRepoError},
            Board,
            BoardId,
            BoardName,
        },
        teams::TeamId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UpdateBoardInput {
    pub id:      Uuid,
    pub name:    String,
    pub team_id: Uuid,
}

pub struct UpdateBoardUseCase {
    board_repo: Arc<dyn BoardRepo>,
}

impl UpdateBoardUseCase {
    pub fn new(board_repo: Arc<dyn BoardRepo>) -> Self {
        Self { board_repo }
    }

    pub async fn execute(&self, input: UpdateBoardInput) -> Result<Board, BoardAppError> {
        let name    = BoardName::parse(&input.name)?;
        let team_id = TeamId::from(input.team_id);

        let mut board = self.board_repo.find_by_id(team_id, BoardId::from(input.id)).await?.ok_or(BoardRepoError::NotFound)?;
        board.name    = name;
        let board     = self.board_repo.update(&board).await?;
        Ok(board)
    }
}
