use crate::{
    application::todos::errors::TodoAppError,
    domain::{
        boards::{
            ports::{BoardRepo, BoardRepoError},
            BoardId,
        },
        teams::TeamId,
        todos::{ports::TodoRepo, Todo},
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct ListTodosInput {
    pub board_id: Uuid,
    pub team_id:  Uuid,
}

pub struct ListTodosUseCase {
    board_repo: Arc<dyn BoardRepo>,
    todo_repo:  Arc<dyn TodoRepo>,
}

impl ListTodosUseCase {
    pub fn new(board_repo: Arc<dyn BoardRepo>, todo_repo: Arc<dyn TodoRepo>) -> Self {
        Self { board_repo, todo_repo }
    }

    pub async fn execute(&self, input: ListTodosInput) -> Result<Vec<Todo>, TodoAppError> {
        let board_id = BoardId::from(input.board_id);
        let team_id  = TeamId::from(input.team_id);
        self.board_repo.find_by_id(team_id, board_id).await?.ok_or(BoardRepoError::NotFound)?;
        let todos = self.todo_repo.list(team_id, board_id).await?;
        Ok(todos)
    }
}
