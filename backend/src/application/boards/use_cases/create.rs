use crate::{
    application::boards::errors::BoardAppError,
    domain::{
        boards::{ports::BoardRepo, Board, BoardName},
        teams::TeamId,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateBoardInput {
    pub name:    String,
    pub team_id: Uuid,
}

pub struct CreateBoardUseCase {
    board_repo: Arc<dyn BoardRepo>,
}

impl CreateBoardUseCase {
    pub fn new(board_repo: Arc<dyn BoardRepo>) -> Self {
        Self { board_repo }
    }

    pub async fn execute(&self, input: CreateBoardInput) -> Result<Board, BoardAppError> {
        let name  = BoardName::parse(&input.name)?;
        let board = Board::create(TeamId::from(input.team_id), name);
        let board = self.board_repo.create(&board).await?;
        Ok(board)
    }
}
