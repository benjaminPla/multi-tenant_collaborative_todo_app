use crate::{
    application::todos::errors::TodoAppError,
    domain::{
        boards::BoardId,
        teams::TeamId,
        todos::{ports::TodoRepo, TodoId},
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct DeleteTodoInput {
    pub board_id: Uuid,
    pub id:       Uuid,
    pub team_id:  Uuid,
}

pub struct DeleteTodoUseCase {
    todo_repo: Arc<dyn TodoRepo>,
}

impl DeleteTodoUseCase {
    pub fn new(todo_repo: Arc<dyn TodoRepo>) -> Self {
        Self { todo_repo }
    }

    pub async fn execute(&self, input: DeleteTodoInput) -> Result<(), TodoAppError> {
        self.todo_repo
            .delete(TeamId::from(input.team_id), BoardId::from(input.board_id), TodoId::from(input.id))
            .await?;
        Ok(())
    }
}
