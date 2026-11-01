use crate::{
    application::todos::errors::TodoAppError,
    domain::{
        boards::BoardId,
        teams::TeamId,
        todos::{
            ports::{TodoRepo, TodoRepoError},
            Todo,
            TodoId,
        },
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetTodoInput {
    pub board_id: Uuid,
    pub id:       Uuid,
    pub team_id:  Uuid,
}

pub struct GetTodoUseCase {
    todo_repo: Arc<dyn TodoRepo>,
}

impl GetTodoUseCase {
    pub fn new(todo_repo: Arc<dyn TodoRepo>) -> Self {
        Self { todo_repo }
    }

    pub async fn execute(&self, input: GetTodoInput) -> Result<Todo, TodoAppError> {
        let todo = self.todo_repo
            .find_by_id(TeamId::from(input.team_id), BoardId::from(input.board_id), TodoId::from(input.id))
            .await?
            .ok_or(TodoRepoError::NotFound)?;
        Ok(todo)
    }
}
